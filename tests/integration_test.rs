use xivapi::{
    models::character::{Gender, Race},
    models::search::Index,
    models::State,
    prelude::*,
};

use serde::Deserialize;

#[tokio::test]
async fn integration_test() -> Result<(), anyhow::Error> {
    let api = XivApi::new();

    // let res = api
    //   .character_search()
    //   .name("Duvivi Duvi")
    //   .server(World::Adamantoise)
    //   .send()?;

    // let id = res.characters[0].id;

    // let res: CharInfoResult = api
    //   .character(1)
    //   .columns(&["Name", "Server", "Race", "Gender"])
    //   .json()?;

    // FIXME: this is still broken
    let res = api.character(1.into()).send().await?;

    // let res = api.enemy(7537.into()).send()?;
    // let res = api.character(2.into()).send()?;
    // let res = api.free_company_search().name("a").server(World::Adamantoise).send();
    // let res = api.free_company(9233645873504730768.into()).send();
    // let res = api.free_company(9233645873504776755.into()).send();
    // let res = api.linkshell_search()
    //   .name("lala world")
    //   .server(World::Adamantoise)
    //   .send();
    // let res = api.linkshell(20547673299957974.into()).send();

    println!("{:#?}", res);

    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CharInfoResult {
    state: State,
    payload: Either<CharInfo, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CharInfo {
    name: String,
    race: Race,
    gender: Gender,
    server: World,
}
