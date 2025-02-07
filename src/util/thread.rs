use std::sync::LazyLock;

use rayon::ThreadPool;

pub const THREAD_POOL: LazyLock<ThreadPool> = LazyLock::new(|| {
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads.min(4))
        .build()
        .expect("could not build thread pool.")
});

macro_rules! spawn {
    ($future:expr) => {{
        use ::tokio::runtime::Builder;
        use ::tokio::task::LocalSet;

        crate::util::thread::THREAD_POOL.spawn(move || {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();
            let local = LocalSet::new();

            local.spawn_local($future);

            rt.block_on(local);
        })
    }};
}

pub(crate) use spawn;
