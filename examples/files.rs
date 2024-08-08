use anyhow::Result;
use pocketbase_sdk::admin::Admin;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub img: Option<String>,
    pub img2: Option<Vec<String>>,
}

fn main() -> Result<()> {
    env_logger::init();
    let authenticated_admin_client = Admin::new("http://xxxxx")
        .auth_with_password("xxx", "xxx")?;

    let token = authenticated_admin_client.get_file_token()?;

    // dbg!(result);
    let list = authenticated_admin_client
        .records("file_test")
        .list()
        .call::<Product>()?;

    dbg!(&list);


    if let Some(ref imgs) = list.items[0].img2 {
        let file_url = authenticated_admin_client
            .records("file_test")
            .get_file_url(&list.items[0].id, &imgs[0])
            .token(token)
            .call();
        dbg!(file_url);
    }

    Ok(())
}
