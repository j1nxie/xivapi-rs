use reqwest::Client;

use url::Url;

use std::str::FromStr;

pub mod builder;
pub mod error;
pub mod models;
pub mod prelude;
pub mod routes;
pub mod util;

use crate::{
    models::{content, id::*},
    routes::{
        character::CharacterBuilder,
        content::ContentBuilder,
        free_company::FreeCompanyBuilder,
        linkshell::LinkshellBuilder,
        search::{
            character::SearchBuilder as CharacterSearchBuilder,
            free_company::SearchBuilder as FreeCompanySearchBuilder,
            linkshell::SearchBuilder as LinkshellSearchBuilder, SearchBuilder,
        },
    },
};

/// The main driver for accessing XIVAPI.
#[derive(Debug)]
pub struct XivApi {
    client: Client,
    key: Option<String>,
}

impl Default for XivApi {
    fn default() -> Self {
        XivApi::new()
    }
}

impl XivApi {
    /// Create an API driver without an API key.
    pub fn new() -> Self {
        let client = Client::new();
        XivApi { client, key: None }
    }

    /// Create an API driver with an API key.
    ///
    /// The key will be added to every request made with this instance.
    pub fn with_key<S: AsRef<str>>(key: S) -> Self {
        let client = Client::new();
        let key = Some(key.as_ref().to_string());
        XivApi { client, key }
    }

    pub(crate) fn url(&self, path: &str) -> Url {
        let mut base = Url::from_str("https://xivapi.com")
            .unwrap()
            .join(path)
            .unwrap();
        if let Some(ref k) = self.key {
            base.query_pairs_mut().append_pair("key", &k);
        }
        base
    }

    /// Search for a character.
    pub fn character_search(&self) -> CharacterSearchBuilder {
        CharacterSearchBuilder::new(self)
    }

    /// Fetch a specific character by their Lodestone ID.
    pub fn character(&self, id: CharacterId) -> CharacterBuilder {
        CharacterBuilder::new(self, id)
    }

    /// Search for a free company.
    pub fn free_company_search(&self) -> FreeCompanySearchBuilder {
        FreeCompanySearchBuilder::new(self)
    }

    /// Fetch a specific free company by its ID.
    pub fn free_company(&self, id: FreeCompanyId) -> FreeCompanyBuilder {
        FreeCompanyBuilder::new(self, id)
    }

    /// Search for a linkshell.
    pub fn linkshell_search(&self) -> LinkshellSearchBuilder {
        LinkshellSearchBuilder::new(self)
    }

    /// Fetch a specific linkshell by its ID.
    pub fn linkshell(&self, id: LinkshellId) -> LinkshellBuilder {
        LinkshellBuilder::new(self, id)
    }

    /// Search for game content.
    pub fn search(&self) -> SearchBuilder {
        SearchBuilder::new(self)
    }

    /// Fetch a specific action by its ID.
    pub fn action(&self, id: ActionId) -> ContentBuilder<content::Action> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific item by its ID.
    pub fn item(&self, id: ItemId) -> ContentBuilder<content::Item> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific achievement by its ID.
    pub fn achievement(&self, id: AchievementId) -> ContentBuilder<content::Achievement> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific emote by its ID.
    pub fn emote(&self, id: EmoteId) -> ContentBuilder<content::Emote> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific enemy by its ID.
    pub fn enemy(&self, id: EnemyId) -> ContentBuilder<content::Enemy> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific fate by its ID.
    pub fn fate(&self, id: FateId) -> ContentBuilder<content::Fate> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch specific instance content by its ID.
    pub fn instance_content(
        &self,
        id: InstanceContentId,
    ) -> ContentBuilder<content::InstanceContent> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific leve by its ID.
    pub fn leve(&self, id: LeveId) -> ContentBuilder<content::Leve> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific minion by its ID.
    pub fn minion(&self, id: MinionId) -> ContentBuilder<content::Minion> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific mount by its ID.
    pub fn mount(&self, id: MountId) -> ContentBuilder<content::Mount> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific NPC by its ID.
    pub fn npc(&self, id: NpcId) -> ContentBuilder<content::Npc> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific place name by its ID.
    pub fn place_name(&self, id: PlaceNameId) -> ContentBuilder<content::PlaceName> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific quest by its ID.
    pub fn quest(&self, id: QuestId) -> ContentBuilder<content::Quest> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific recipe by its ID.
    pub fn recipe(&self, id: RecipeId) -> ContentBuilder<content::Recipe> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific status by its ID.
    pub fn status(&self, id: StatusId) -> ContentBuilder<content::Status> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific title by its ID.
    pub fn title(&self, id: TitleId) -> ContentBuilder<content::Title> {
        ContentBuilder::new(self, id.0)
    }

    /// Fetch a specific weather by its ID.
    pub fn weather(&self, id: WeatherId) -> ContentBuilder<content::Weather> {
        ContentBuilder::new(self, id.0)
    }
}
