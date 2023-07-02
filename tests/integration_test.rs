use xivapi::{
    models::character::{Gender, Race},
    prelude::*,
};

use serde::Deserialize;

#[tokio::test]
async fn integration_test() -> Result<(), anyhow::Error> {
    let api = XivApi::new();

    // let res = api
    // .character_search()
    // .name("Lumine Vernilet")
    // .server(World::Typhon)
    // .send()
    // .await?;

    // let id = res.characters[0].id;

    let res: CharInfoResult = api
        .character(31546648.into())
        .columns(&[
            "Character.Name",
            "Character.Server",
            "Character.Race",
            "Character.Gender",
        ])
        .json()
        .await?;

    // let res = api.character(31546648.into()).send().await?;

    // let res = api.enemy(7537.into()).send().await?;
    // let res = api.character(2.into()).send()?;
    // let res = api
    // .free_company_search()
    // .name("Mirai")
    // .server(World::Typhon)
    // .send()
    // .await;
    // let res = api.free_company(9230408911272603212.into()).send().await;
    // let res = api.free_company(9233645873504776755.into()).send();
    // let res = api
    // .linkshell_search()
    // .name("lala world")
    // .server(World::Adamantoise)
    // .send()
    // .await?;
    // let res = api.linkshell(20547673299957974.into()).send().await?;

    println!("{:#?}", res);

    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CharInfoResult {
    #[serde(rename = "Character")]
    payload: CharInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CharInfo {
    name: String,
    race: Race,
    gender: Gender,
    server: World,
}
