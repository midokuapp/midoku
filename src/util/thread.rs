macro_rules! spawn {
    ($future:expr) => {{
        use ::tokio::runtime::Builder;
        use ::tokio::task::LocalSet;

        use crate::THREAD_POOL;

        THREAD_POOL.spawn(move || {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();
            let local = LocalSet::new();

            local.spawn_local($future);

            rt.block_on(local);
        })
    }};
}

pub(crate) use spawn;
