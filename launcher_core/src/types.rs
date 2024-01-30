use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

impl VersionManifest {
    pub fn latest_release(&self) -> &Version {
        for version in &self.versions {
            if version.id == self.latest.release {
                return version;
            }
        }

        // If the latest release does not exist in the meta, things have probably gone wrong lol
        unreachable!()
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: Type,
    pub url: String,
    pub time: String,
    pub release_time: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    OldAlpha,
    OldBeta,
    Release,
    Snapshot,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct VersionJson {
    #[serde(alias = "minecraftArguments")]
    pub arguments: Arguments,
    pub asset_index: AssetIndex,
    pub assets: String,
    pub compliance_level: Option<i64>,
    pub downloads: Box<WelcomeDownloads>,
    pub id: String,
    pub java_version: Option<JavaVersion>,
    pub logging: Option<Logging>,
    pub main_class: String,
    pub minimum_launcher_version: i64,
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub welcome_type: String,
    pub libraries: Vec<Library>,
}

impl VersionJson {
    /// Shorthand for matching and getting the ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Refers to the client jar url
    pub fn url(&self) -> &str {
        &self.downloads.client.url
    }

    /// Refers to the client jar sha1
    pub fn sha1(&self) -> &str {
        &self.downloads.client.sha1
    }

    pub fn libraries(&self) -> &Vec<Library> {
        &self.libraries
    }

    pub fn asset_index(&self) -> &AssetIndex {
        &self.asset_index
    }

    pub fn release_type(&self) -> &str {
        &self.welcome_type
    }

    pub fn main_class(&self) -> &str {
        &self.main_class
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i64,
    pub total_size: Option<i64>,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    pub rules: Option<Vec<Rule>>,
    pub extract: Option<Extract>,
    pub natives: Option<Natives>,
}

impl Natives {
    pub fn applies(&self) -> bool {
        #[cfg(windows)]
        return self.windows.is_some();
        #[cfg(target_os = "linux")]
        return self.linux.is_some();
        #[cfg(target_os = "mac_os")]
        return self.osx.is_some();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Natives {
    pub linux: Option<String>,
    pub osx: Option<String>,
    pub windows: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Extract {
    pub exclude: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    pub action: Action,
    pub os: Option<Os>,
}

impl Rule {
    pub fn applies(&self) -> bool {
        if let Some(os) = &self.os {
            os.name == OS && self.action == Action::Allow
        } else {
            self.action == Action::Allow
        }
    }

    pub fn native(&self) -> bool {
        if let Some(os) = &self.os {
            if os.name == OS && self.action == Action::Allow {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum Action {
    Allow,
    Disallow,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Os {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryDownloads {
    pub artifact: Option<Artifact>,
    pub classifiers: Option<Classifiers>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Classifiers {
    #[serde(rename = "linux-x86_64")]
    pub linux_x86_64: Option<Artifact>,
    pub natives_linux: Option<Artifact>,
    pub natives_macos: Option<Artifact>,
    pub natives_osx: Option<Artifact>,
    pub natives_windows: Option<Artifact>,
    pub natives_windows_32: Option<Artifact>,
    pub natives_windows_64: Option<Artifact>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub sha1: String,
    pub size: u64,
    pub url: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssetIndexJson {
    #[serde(skip)]
    #[allow(unused)]
    map_to_resources: Option<bool>,
    pub objects: std::collections::HashMap<String, Object>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub hash: String,
    pub size: u64,
}

#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Arguments {
    pub game: Vec<GameElement>,
    pub jvm: Option<Vec<JvmClass>>,
}

impl<'de> Deserialize<'de> for Arguments {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct TempArguments {
            pub game: Vec<GameElement>,
            pub jvm: Option<Vec<JvmClass>>,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum TempArgs {
            Args(TempArguments),
            String(String),
        }

        let v = TempArgs::deserialize(deserializer)?;

        let r = match v {
            TempArgs::Args(t) => Arguments { jvm: t.jvm, game: t.game },
            TempArgs::String(s) => {
                Arguments {
                    jvm: None,
                    game: s.split(' ').map(|s| {GameElement::String(s.to_owned())}).collect()
                }
            }
        };

        Ok(r)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum GameElement {
    GameClass(GameClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameClass {
    pub rules: Option<Vec<GameRule>>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub value: Box<[String]>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameRule {
    pub action: Action,
    pub features: Features,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Features {
    pub is_demo_user: Option<bool>,
    pub has_custom_resolution: Option<bool>,
    pub has_quick_plays_support: Option<bool>,
    pub is_quick_play_singleplayer: Option<bool>,
    pub is_quick_play_multiplayer: Option<bool>,
    pub is_quick_play_realms: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JvmClass {
    pub rules: Option<Vec<JvmRule>>,
    pub value: Box<[String]>,
}

impl<'de> serde::de::Deserialize<'de> for JvmClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        #[derive(Deserialize)]
        struct Temp {
            pub rules: Option<Vec<JvmRule>>,
            #[serde(deserialize_with = "string_or_seq_string")]
            pub value: Box<[String]>,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum TempClass {
            Class(Temp),
            String(String),
        }

        let v = TempClass::deserialize(deserializer)?;

        Ok(
            match v {
                TempClass::Class(c) => JvmClass { value: c.value, rules: c.rules },
                TempClass::String(s) => JvmClass { value: Box::new([s]), rules: None }
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JvmRule {
    pub action: Action,
    pub os: PurpleOs,
}

impl JvmRule {
    pub fn applies(&self) -> bool {
        if let Some(os) = &self.os.name {
            os == OS && self.action == Action::Allow
        } else {
            self.action == Action::Allow
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PurpleOs {
    pub name: Option<String>,
    pub arch: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WelcomeDownloads {
    pub client: Jar,
    pub client_mappings: Option<Jar>,
    pub server: Option<Jar>,
    pub server_mappings: Option<Jar>,
    pub windows_server: Option<Jar>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Jar {
    pub sha1: String,
    pub size: u64,
    pub url: String,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct JavaVersion {
    pub component: String,
    pub major_version: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Logging {
    pub client: LoggingClient,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoggingClient {
    pub argument: String,
    pub file: AssetIndex,
    #[serde(rename = "type")]
    pub client_type: String,
}

use crate::OS;

fn string_or_seq_string<'de, D>(deserializer: D) -> Result<Box<[String]>, D::Error>
    where D: serde::Deserializer<'de>
{
    struct StringOrBoxArray(std::marker::PhantomData<Box<[String]>>);

    impl<'de> serde::de::Visitor<'de> for StringOrBoxArray {
        type Value = Box<[String]>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: serde::de::Error
        {
            Ok(Box::new([value.to_owned()]))
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
            where S: serde::de::SeqAccess<'de>
        {
            Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrBoxArray(std::marker::PhantomData))
}
