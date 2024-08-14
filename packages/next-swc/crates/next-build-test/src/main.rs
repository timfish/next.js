use std::{convert::Infallible, str::FromStr};

use next_api::project::{DefineEnv, ProjectOptions};
use next_build_test::{main_inner, Strategy};
use turbo_tasks::TurboTasks;
use turbo_tasks_malloc::TurboMalloc;
use turbopack_binding::turbo::tasks_memory::MemoryBackend;

enum Cmd {
    Run,
    Generate,
}
impl FromStr for Cmd {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "run" => Ok(Cmd::Run),
            "generate" => Ok(Cmd::Generate),
            _ => panic!("invalid command, please use 'run' or 'generate'"),
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let cmd = std::env::args()
        .nth(1)
        .map(|s| Cmd::from_str(&s))
        .unwrap_or(Ok(Cmd::Run))
        .unwrap();

    match cmd {
        Cmd::Run => {
            let strat = std::env::args()
                .nth(2)
                .map(|s| Strategy::from_str(&s))
                .transpose()
                .unwrap()
                .unwrap_or(Strategy::Sequential);

            let mut factor = std::env::args()
                .nth(3)
                .map(|s| s.parse().unwrap())
                .unwrap_or(num_cpus::get());

            let limit = std::env::args()
                .nth(4)
                .map(|s| s.parse().unwrap())
                .unwrap_or(1);

            let files = std::env::args()
                .nth(5)
                .map(|f| f.split(',').map(ToOwned::to_owned).collect());

            if strat == Strategy::Sequential {
                factor = 1;
            }

            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .on_thread_stop(|| {
                    TurboMalloc::thread_stop();
                    tracing::debug!("threads stopped");
                })
                .build()
                .unwrap()
                .block_on(async {
                    let tt = TurboTasks::new(MemoryBackend::new(usize::MAX));
                    let x = tt.run_once(main_inner(strat, factor, limit, files)).await;
                    tracing::debug!("done");
                    x
                })
                .unwrap();
        }
        Cmd::Generate => {
            let project_path = std::env::args().nth(2).unwrap_or(".".to_string());
            let current_dir = std::env::current_dir().unwrap();
            let absolute_dir = current_dir.join(project_path);
            let canonical_path = std::fs::canonicalize(absolute_dir).unwrap();

            let options = ProjectOptions {
                build_id: "test".to_owned(),
                define_env: DefineEnv {
                    client: vec![],
                    edge: vec![],
                    nodejs: vec![],
                },
                dev: true,
                encryption_key: "deadbeef".to_string(),
                env: vec![],
                js_config: include_str!("../jsConfig.json").to_string(),
                next_config: include_str!("../nextConfig.json").to_string(),
                preview_props: next_api::project::DraftModeOptions {
                    preview_mode_encryption_key: "deadbeef".to_string(),
                    preview_mode_id: "test".to_string(),
                    preview_mode_signing_key: "deadbeef".to_string(),
                },
                project_path: canonical_path.to_string_lossy().to_string(),
                root_path: "/".to_string(),
                watch: false,
            };

            let json = serde_json::to_string_pretty(&options).unwrap();
            println!("{}", json);
        }
    }
}
