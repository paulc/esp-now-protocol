use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use rquickjs::{async_with, AsyncContext, AsyncRuntime};

use rquickjs_utils::repl::repl_rl;
use rquickjs_utils::run::{call_fn, get_script, run_module, run_script};
use rquickjs_utils::utils::{json_to_value, register_fns, value_to_json};

use argh::FromArgs;

use tokio::signal::ctrl_c;

use esp_now_protocol::register_espnow;

#[derive(FromArgs)]
/// CLI Args
struct CliArgs {
    #[argh(option)]
    /// QJS script
    script: Vec<String>,
    #[argh(option)]
    /// QJS module
    module: Vec<String>,
    #[argh(switch)]
    /// JS REPL
    repl: bool,
    #[argh(option)]
    /// call JS
    call: Vec<String>,
    #[argh(option)]
    /// call args
    arg: Vec<String>,
}

static USER_EXIT: AtomicBool = AtomicBool::new(false);
static MSG_ID: AtomicU32 = AtomicU32::new(0);

#[rquickjs::function]
pub fn next_id() -> u32 {
    MSG_ID.fetch_add(1, Ordering::Relaxed)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get CLI args
    let args: CliArgs = argh::from_env();

    // Check that we have something to do
    if args.script.is_empty() && args.module.is_empty() && args.call.is_empty() && !args.repl {
        let name = std::env::args().next().unwrap_or("-".into());
        CliArgs::from_args(&[&name], &["--help"]).map_err(|exit| anyhow::anyhow!(exit.output))?;
    }

    // Start task waiting for Ctrl-C
    tokio::spawn(async move {
        ctrl_c().await.expect("Error listening for Ctrl-C");
        println!("[+] User Exit",);
        USER_EXIT.store(true, Ordering::Relaxed);
    });

    let rt = AsyncRuntime::new()?;
    let ctx = AsyncContext::full(&rt).await?;

    // Set interrupt handler - this only seems to be called on ctx.eval() so not actually useful
    rt.set_interrupt_handler(Some(Box::new(|| USER_EXIT.load(Ordering::Relaxed))))
        .await;

    async_with!(ctx => |ctx| {
        register_fns(&ctx)?;
        register_espnow(&ctx)?;
        ctx.globals().set("next_id", js_next_id)?;

        // Run modules
        for module in args.module {
            run_module(ctx.clone(),get_script(&module)?).await?;
        }

        // Run scripts
        for script in args.script {
            run_script(ctx.clone(),get_script(&script)?).await?;
        }

        // Run REPL
        if args.repl {
            repl_rl(ctx.clone()).await?;
        }

        // Call JS
        for (f,a) in args.call.iter().zip(args.arg.iter().chain(std::iter::repeat(&("".to_string())))) {
            let r = if a.is_empty() {
                call_fn(ctx.clone(),&f,((),)).await?
            } else {
                call_fn(ctx.clone(),&f,(json_to_value(ctx.clone(),a)?,)).await?
            };
            println!("[+] Call: {f}({a}) => {}", value_to_json(ctx.clone(),r)?);
        }

        Ok::<(),anyhow::Error>(())
    })
    .await?;

    println!("[+] Tasks Pending: {:?}", rt.is_job_pending().await);

    // Complete pending tasks - use rt.execute_pending_job() rather than rt.idle() to allow
    // USER_EXIT to interrupt
    while rt.is_job_pending().await && !USER_EXIT.load(Ordering::Relaxed) {
        rt.execute_pending_job()
            .await
            .map_err(|_| anyhow::anyhow!("JS Runtime Error"))?;
        // Make sure we yield (possibly not necessary)
        tokio::task::yield_now().await;
    }

    Ok(())
}
