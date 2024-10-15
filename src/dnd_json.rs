// Generated using https://app.quicktype.io/#l=rust 
//
// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Character;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Character = serde_json::from_str(&json).unwrap();
// }

pub mod dnd_json {

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterJson {
    pub id: i64,
    pub success: bool,
    pub message: String,
    pub data: Data,
    pub pagination: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    id: i64,
    user_id: i64,
    pub username: String,
    is_assigned_to_player: bool,
    readonly_url: String,
    decorations: Decorations,
    pub name: String,
    social_name: Option<serde_json::Value>,
    gender: Option<serde_json::Value>,
    faith: Option<serde_json::Value>,
    age: Option<serde_json::Value>,
    hair: Option<serde_json::Value>,
    eyes: Option<serde_json::Value>,
    skin: Option<serde_json::Value>,
    height: Option<serde_json::Value>,
    weight: Option<serde_json::Value>,
    inspiration: bool,
    pub base_hit_points: i64,
    bonus_hit_points: Option<serde_json::Value>,
    override_hit_points: Option<serde_json::Value>,
    removed_hit_points: i64,
    temporary_hit_points: i64,
    current_xp: i64,
    alignment_id: Option<serde_json::Value>,
    lifestyle_id: Option<serde_json::Value>,
    pub stats: Vec<Stat>,
    pub bonus_stats: Vec<Stat>,
    pub override_stats: Vec<Stat>,
    background: DataBackground,
    pub race: Race,
    race_definition_id: Option<serde_json::Value>,
    race_definition_type_id: Option<serde_json::Value>,
    notes: Notes,
    traits: Traits,
    preferences: Preferences,
    configuration: Configuration,
    lifestyle: Option<serde_json::Value>,
    inventory: Vec<Inventory>,
    currencies: Currencies,
    pub classes: Vec<DataClass>,
    feats: Vec<DataFeat>,
    features: Option<serde_json::Value>,
    custom_defense_adjustments: Vec<Option<serde_json::Value>>,
    custom_senses: Vec<Option<serde_json::Value>>,
    custom_speeds: Vec<Option<serde_json::Value>>,
    custom_proficiencies: Vec<Option<serde_json::Value>>,
    custom_actions: Vec<Option<serde_json::Value>>,
    character_values: Vec<Option<serde_json::Value>>,
    conditions: Vec<Option<serde_json::Value>>,
    death_saves: DeathSaves,
    adjustment_xp: Option<serde_json::Value>,
    pub spell_slots: Vec<PactMagic>,
    pub pact_magic: Vec<PactMagic>,
    active_source_categories: Vec<i64>,
    spells: Actions,
    options: Actions,
    pub choices: Choices,
    actions: Actions,
    pub modifiers: Modifiers,
    class_spells: Vec<ClassSpell>,
    custom_items: Vec<CustomItem>,
    campaign: Campaign,
    creatures: Vec<Option<serde_json::Value>>,
    optional_origins: Vec<Option<serde_json::Value>>,
    optional_class_features: Vec<Option<serde_json::Value>>,
    date_modified: String,
    provided_from: String,
    can_edit: bool,
    status: i64,
    status_slug: Option<serde_json::Value>,
    campaign_setting: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actions {
    race: Vec<Option<serde_json::Value>>,
    class: Vec<ActionsClass>,
    background: Option<serde_json::Value>,
    item: Option<Vec<Item>>,
    feat: Vec<ActionsFeat>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionsClass {
    component_id: i64,
    component_type_id: i64,
    id: String,
    entity_type_id: String,
    limited_use: Option<ClassLimitedUse>,
    name: String,
    description: Option<String>,
    snippet: String,
    ability_modifier_stat_id: Option<serde_json::Value>,
    on_miss_description: Option<String>,
    save_fail_description: Option<String>,
    save_success_description: Option<String>,
    save_stat_id: Option<serde_json::Value>,
    fixed_save_dc: Option<serde_json::Value>,
    attack_type_range: Option<serde_json::Value>,
    action_type: i64,
    attack_subtype: Option<serde_json::Value>,
    dice: Option<serde_json::Value>,
    value: Option<serde_json::Value>,
    damage_type_id: Option<serde_json::Value>,
    is_martial_arts: bool,
    is_proficient: bool,
    spell_range_type: Option<serde_json::Value>,
    display_as_attack: Option<serde_json::Value>,
    range: ClassRange,
    activation: Activation,
    number_of_targets: Option<serde_json::Value>,
    fixed_to_hit: Option<serde_json::Value>,
    ammunition: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activation {
    activation_time: Option<i64>,
    activation_type: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassLimitedUse {
    name: Option<serde_json::Value>,
    stat_modifier_uses_id: Option<i64>,
    reset_type: Option<i64>,
    number_used: i64,
    min_number_consumed: Option<i64>,
    max_number_consumed: i64,
    max_uses: i64,
    operator: Option<i64>,
    use_proficiency_bonus: bool,
    proficiency_bonus_operator: Option<i64>,
    reset_dice: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassRange {
    range: Option<serde_json::Value>,
    long_range: Option<serde_json::Value>,
    aoe_type: Option<serde_json::Value>,
    aoe_size: Option<serde_json::Value>,
    has_aoe_special_description: bool,
    minimum_range: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionsFeat {
    override_save_dc: Option<serde_json::Value>,
    limited_use: Option<ClassLimitedUse>,
    id: serde_json::Value,
    entity_type_id: serde_json::Value,
    definition: Option<ItemDefinition>,
    definition_id: Option<i64>,
    prepared: Option<bool>,
    counts_as_known_spell: Option<bool>,
    uses_spell_slot: Option<bool>,
    cast_at_level: Option<i64>,
    always_prepared: Option<bool>,
    restriction: Option<String>,
    spell_casting_ability_id: Option<i64>,
    display_as_attack: Option<serde_json::Value>,
    additional_description: Option<serde_json::Value>,
    cast_only_as_ritual: Option<bool>,
    ritual_casting_type: Option<serde_json::Value>,
    range: Option<DefinitionRange>,
    activation: Option<Activation>,
    base_level_at_will: Option<bool>,
    at_will_limited_use_level: Option<serde_json::Value>,
    is_signature_spell: Option<serde_json::Value>,
    component_id: Option<i64>,
    component_type_id: Option<i64>,
    spell_list_id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDefinition {
    id: i64,
    definition_key: String,
    name: String,
    level: i64,
    school: String,
    duration: Duration,
    activation: Activation,
    range: DefinitionRange,
    as_part_of_weapon_attack: bool,
    description: String,
    snippet: String,
    concentration: bool,
    ritual: bool,
    range_area: Option<serde_json::Value>,
    damage_effect: Option<serde_json::Value>,
    components: Vec<i64>,
    components_description: String,
    save_dc_ability_id: Option<i64>,
    healing: Option<serde_json::Value>,
    healing_dice: Vec<Option<serde_json::Value>>,
    temp_hp_dice: Vec<Option<serde_json::Value>>,
    attack_type: Option<i64>,
    can_cast_at_higher_level: bool,
    is_homebrew: bool,
    version: Option<serde_json::Value>,
    source_id: Option<serde_json::Value>,
    source_page_number: i64,
    requires_saving_throw: bool,
    requires_attack_roll: bool,
    at_higher_levels: AtHigherLevels,
    modifiers: Vec<ItemElement>,
    conditions: Vec<Condition>,
    tags: Vec<String>,
    casting_time_description: String,
    scale_type: Option<ScaleType>,
    sources: Vec<Source>,
    spell_groups: Vec<i64>,
    is_legacy: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtHigherLevels {
    higher_level_definitions: Vec<HigherLevelDefinition>,
    additional_attacks: Vec<Option<serde_json::Value>>,
    additional_targets: Vec<Option<serde_json::Value>>,
    area_of_effect: Vec<Option<serde_json::Value>>,
    duration: Vec<Option<serde_json::Value>>,
    creatures: Vec<Option<serde_json::Value>>,
    special: Vec<Option<serde_json::Value>>,
    points: Vec<Option<serde_json::Value>>,
    range: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HigherLevelDefinition {
    level: Option<i64>,
    type_id: i64,
    dice: Option<WealthDice>,
    value: Option<i64>,
    details: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WealthDice {
    dice_count: Option<i64>,
    dice_value: Option<i64>,
    dice_multiplier: Option<i64>,
    fixed_value: Option<i64>,
    dice_string: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    #[serde(rename = "type")]
    condition_type: i64,
    condition_id: i64,
    condition_duration: i64,
    duration_unit: DurationUnit,
    exception: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DurationUnit {
    Hour,
    Minute,
    Round,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Duration {
    duration_interval: i64,
    duration_unit: Option<DurationUnit>,
    duration_type: DurationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DurationType {
    Concentration,
    Instantaneous,
    Time,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemElement {
    pub fixed_value: Option<i64>,
    pub id: String,
    pub entity_id: Option<i64>,
    pub entity_type_id: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub background_type: String,
    pub sub_type: String,
    pub dice: Option<serde_json::Value>,
    pub restriction: Option<String>,
    pub stat_id: Option<serde_json::Value>,
    pub requires_attunement: bool,
    pub duration: Option<serde_json::Value>,
    pub friendly_type_name: String,
    pub friendly_subtype_name: String,
    pub is_granted: bool,
    pub bonus_types: Vec<Option<serde_json::Value>>,
    pub value: Option<i64>,
    pub available_to_multiclass: Option<bool>,
    pub modifier_type_id: i64,
    pub modifier_sub_type_id: i64,
    pub component_id: i64,
    pub component_type_id: i64,
    pub die: Option<WealthDice>,
    pub count: Option<i64>,
    pub duration_unit: Option<serde_json::Value>,
    pub use_primary_stat: Option<bool>,
    pub at_higher_levels: Option<AtHigherLevels>,
    pub tag_constraints: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionRange {
    origin: Option<Origin>,
    range_value: Option<i64>,
    aoe_type: Option<String>,
    aoe_value: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Origin {
    #[serde(rename = "Self")]
    OriginSelf,
    Ranged,
    Touch,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaleType {
    Characterlevel,
    Spellscale,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    source_id: i64,
    page_number: Option<i64>,
    source_type: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    override_save_dc: Option<serde_json::Value>,
    limited_use: Option<ClassLimitedUse>,
    id: serde_json::Value,
    entity_type_id: serde_json::Value,
    definition: ItemDefinition,
    definition_id: i64,
    prepared: bool,
    counts_as_known_spell: Option<bool>,
    uses_spell_slot: bool,
    cast_at_level: Option<serde_json::Value>,
    always_prepared: bool,
    restriction: Option<serde_json::Value>,
    spell_casting_ability_id: Option<serde_json::Value>,
    display_as_attack: Option<bool>,
    additional_description: Option<serde_json::Value>,
    cast_only_as_ritual: bool,
    ritual_casting_type: Option<serde_json::Value>,
    range: DefinitionRange,
    activation: Activation,
    base_level_at_will: bool,
    at_will_limited_use_level: Option<serde_json::Value>,
    is_signature_spell: Option<serde_json::Value>,
    component_id: i64,
    component_type_id: i64,
    spell_list_id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataBackground {
    has_custom_background: bool,
    definition: BackgroundDefinition,
    definition_id: Option<serde_json::Value>,
    custom_background: CustomBackground,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomBackground {
    id: i64,
    entity_type_id: i64,
    name: Option<serde_json::Value>,
    description: Option<serde_json::Value>,
    features_background: Option<serde_json::Value>,
    characteristics_background: Option<serde_json::Value>,
    features_background_definition_id: Option<serde_json::Value>,
    characteristics_background_definition_id: Option<serde_json::Value>,
    background_type: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundDefinition {
    id: i64,
    entity_type_id: i64,
    definition_key: String,
    name: String,
    description: String,
    snippet: String,
    short_description: String,
    skill_proficiencies_description: String,
    tool_proficiencies_description: String,
    languages_description: String,
    equipment_description: String,
    feature_name: String,
    feature_description: String,
    avatar_url: Option<serde_json::Value>,
    large_avatar_url: Option<serde_json::Value>,
    suggested_characteristics_description: String,
    suggested_proficiencies: Option<serde_json::Value>,
    suggested_languages: Option<serde_json::Value>,
    organization: Option<serde_json::Value>,
    contracts_description: String,
    spells_pre_description: String,
    spells_post_description: String,
    personality_traits: Vec<Bond>,
    ideals: Vec<Bond>,
    bonds: Vec<Bond>,
    flaws: Vec<Bond>,
    is_homebrew: bool,
    sources: Vec<Source>,
    spell_list_ids: Vec<Option<serde_json::Value>>,
    feat_list: Option<serde_json::Value>,
    granted_feats: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bond {
    id: i64,
    description: String,
    dice_roll: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    id: i64,
    name: Option<serde_json::Value>,
    pub value: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Campaign {
    id: i64,
    name: String,
    description: String,
    link: String,
    public_notes: String,
    dm_user_id: i64,
    dm_username: String,
    characters: Vec<CharacterElement>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterElement {
    user_id: i64,
    username: String,
    character_id: i64,
    character_name: String,
    character_url: String,
    avatar_url: Option<String>,
    privacy_type: i64,
    campaign_id: Option<serde_json::Value>,
    is_assigned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choices {
    pub race: Vec<FeatElement>,
    pub class: Vec<FeatElement>,
    pub background: Vec<FeatElement>,
    item: Option<serde_json::Value>,
    pub feat: Vec<FeatElement>,
    pub choice_definitions: Vec<ChoiceDefinition>,
    definition_key_name_map: DefinitionKeyNameMap,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatElement {
    component_id: i64,
    component_type_id: i64,
    id: String,
    parent_choice_id: Option<String>,
    #[serde(rename = "type")]
    pub background_type: i64,
    pub sub_type: Option<i64>,
    pub option_value: i64,
    label: Option<String>,
    is_optional: bool,
    is_infinite: bool,
    default_subtypes: Vec<String>,
    display_order: Option<serde_json::Value>,
    options: Vec<Option<serde_json::Value>>,
    option_ids: Vec<i64>,
    tag_constraints: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChoiceDefinition {
    id: String,
    pub options: Vec<Options>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub id: i64,
    pub label: String,
    description: Option<String>,
    source_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefinitionKeyNameMap {
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassSpell {
    entity_type_id: i64,
    character_class_id: i64,
    spells: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataClass {
    id: i64,
    entity_type_id: i64,
    pub level: i64,
    is_starting_class: bool,
    hit_dice_used: i64,
    definition_id: i64,
    subclass_definition_id: Option<serde_json::Value>,
    pub definition: SubclassDefinitionClass,
    subclass_definition: SubclassDefinitionClass,
    class_features: Vec<ClassClassFeature>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassClassFeature {
    definition: ClassFeatureDefinition,
    level_scale: Option<LevelScale>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassFeatureDefinition {
    id: serde_json::Value,
    definition_key: String,
    entity_type_id: serde_json::Value,
    display_order: i64,
    name: String,
    description: String,
    snippet: String,
    activation: Option<serde_json::Value>,
    multi_class_description: String,
    required_level: i64,
    is_sub_class_feature: bool,
    limited_use: Vec<LimitedUseElement>,
    hide_in_builder: bool,
    hide_in_sheet: bool,
    source_id: i64,
    source_page_number: Option<i64>,
    creature_rules: Vec<Option<serde_json::Value>>,
    level_scales: Vec<LevelScale>,
    infusion_rules: Vec<Option<serde_json::Value>>,
    spell_list_ids: Vec<Option<serde_json::Value>>,
    class_id: i64,
    feature_type: i64,
    sources: Vec<Source>,
    affected_feature_definition_keys: Vec<Option<serde_json::Value>>,
    entity_type: EntityType,
    #[serde(rename = "entityID")]
    entity_id: String,
    granted_feats: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EntityType {
    #[serde(rename = "class-feature")]
    ClassFeature,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelScale {
    id: i64,
    level: i64,
    description: String,
    dice: WealthDice,
    fixed_value: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitedUseElement {
    level: Option<serde_json::Value>,
    uses: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubclassDefinitionClass {
    id: i64,
    definition_key: String,
    pub name: String,
    description: String,
    equipment_description: Option<String>,
    parent_class_id: Option<i64>,
    avatar_url: Option<String>,
    large_avatar_url: Option<String>,
    portrait_avatar_url: Option<String>,
    more_details_url: String,
    spell_casting_ability_id: i64,
    sources: Vec<Source>,
    class_features: Vec<DefinitionClassFeature>,
    hit_dice: i64,
    wealth_dice: Option<WealthDice>,
    can_cast_spells: bool,
    knows_all_spells: bool,
    spell_prepare_type: Option<serde_json::Value>,
    spell_container_name: Option<serde_json::Value>,
    source_page_number: i64,
    subclass_definition: Option<serde_json::Value>,
    is_homebrew: bool,
    primary_abilities: Option<Vec<i64>>,
    spell_rules: Option<SpellRules>,
    prerequisites: Option<Vec<Prerequisite>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionClassFeature {
    id: i64,
    name: String,
    prerequisite: Option<serde_json::Value>,
    description: String,
    required_level: i64,
    display_order: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prerequisite {
    description: String,
    prerequisite_mappings: Vec<PrerequisiteMapping>,
    hide_prerequisite: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrerequisiteMapping {
    id: i64,
    entity_id: i64,
    entity_type_id: i64,
    #[serde(rename = "type")]
    prerequisite_mapping_type: String,
    sub_type: String,
    value: i64,
    friendly_type_name: String,
    friendly_sub_type_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellRules {
    multi_class_spell_slot_divisor: i64,
    is_ritual_spell_caster: bool,
    level_cantrips_known_maxes: Vec<i64>,
    level_spell_known_maxes: Vec<i64>,
    level_spell_slots: Vec<Vec<i64>>,
    multi_class_spell_slot_rounding: i64,
    level_prepared_spell_maxes: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    starting_equipment_type: i64,
    ability_score_type: i64,
    show_help_text: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currencies {
    cp: i64,
    sp: i64,
    gp: i64,
    ep: i64,
    pp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomItem {
    id: i64,
    name: String,
    description: Option<serde_json::Value>,
    weight: Option<serde_json::Value>,
    cost: Option<serde_json::Value>,
    quantity: i64,
    notes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeathSaves {
    fail_count: Option<i64>,
    success_count: Option<i64>,
    is_stabilized: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decorations {
    avatar_url: String,
    frame_avatar_url: Option<serde_json::Value>,
    backdrop_avatar_url: Option<serde_json::Value>,
    small_backdrop_avatar_url: Option<serde_json::Value>,
    large_backdrop_avatar_url: Option<serde_json::Value>,
    thumbnail_backdrop_avatar_url: Option<serde_json::Value>,
    default_backdrop: DefaultBackdrop,
    avatar_id: i64,
    portrait_decoration_key: Option<serde_json::Value>,
    frame_avatar_decoration_key: Option<serde_json::Value>,
    frame_avatar_id: Option<serde_json::Value>,
    backdrop_avatar_decoration_key: Option<serde_json::Value>,
    backdrop_avatar_id: Option<serde_json::Value>,
    small_backdrop_avatar_decoration_key: String,
    small_backdrop_avatar_id: Option<serde_json::Value>,
    large_backdrop_avatar_decoration_key: String,
    large_backdrop_avatar_id: Option<serde_json::Value>,
    thumbnail_backdrop_avatar_decoration_key: String,
    thumbnail_backdrop_avatar_id: Option<serde_json::Value>,
    theme_color: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultBackdrop {
    backdrop_avatar_url: String,
    small_backdrop_avatar_url: String,
    large_backdrop_avatar_url: String,
    thumbnail_backdrop_avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataFeat {
    component_type_id: i64,
    component_id: i64,
    definition: PurpleDefinition,
    definition_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleDefinition {
    id: i64,
    entity_type_id: i64,
    definition_key: String,
    name: String,
    description: String,
    snippet: String,
    activation: Activation,
    source_id: Option<serde_json::Value>,
    source_page_number: Option<serde_json::Value>,
    creature_rules: Vec<Option<serde_json::Value>>,
    prerequisites: Vec<Option<serde_json::Value>>,
    is_homebrew: bool,
    sources: Vec<Source>,
    spell_list_ids: Vec<Option<serde_json::Value>>,
    is_repeatable: bool,
    repeatable_parent_id: Option<serde_json::Value>,
    categories: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    id: serde_json::Value,
    entity_type_id: serde_json::Value,
    definition: InventoryDefinition,
    definition_id: i64,
    definition_type_id: i64,
    display_as_attack: Option<serde_json::Value>,
    quantity: i64,
    is_attuned: bool,
    equipped: bool,
    equipped_entity_type_id: Option<i64>,
    equipped_entity_id: Option<i64>,
    charges_used: i64,
    limited_use: Option<InventoryLimitedUse>,
    container_entity_id: i64,
    container_entity_type_id: i64,
    container_definition_key: String,
    currency: Option<serde_json::Value>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub enum ContainerDefinitionKey {
//     #[serde(rename = "1439493548:985955090")]
//     The1439493548985955090,
//     #[serde(rename = "1581111423:88796596")]
//     The158111142388796596,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryDefinition {
    id: i64,
    base_type_id: i64,
    entity_type_id: i64,
    definition_key: String,
    can_equip: bool,
    magic: bool,
    name: String,
    snippet: Option<String>,
    weight: f64,
    weight_multiplier: f64,
    capacity: Option<String>,
    capacity_weight: f64,
    #[serde(rename = "type")]
    definition_type: Option<String>,
    description: String,
    can_attune: bool,
    attunement_description: Option<String>,
    rarity: Rarity,
    is_homebrew: bool,
    version: Option<String>,
    source_id: Option<serde_json::Value>,
    source_page_number: Option<serde_json::Value>,
    stackable: bool,
    bundle_size: i64,
    avatar_url: Option<String>,
    large_avatar_url: Option<String>,
    filter_type: FilterType,
    cost: Option<f64>,
    is_pack: bool,
    tags: Vec<String>,
    granted_modifiers: Vec<ItemElement>,
    sub_type: Option<String>,
    is_consumable: bool,
    weapon_behaviors: Vec<Option<serde_json::Value>>,
    base_item_id: Option<i64>,
    base_armor_name: Option<String>,
    strength_requirement: Option<i64>,
    armor_class: Option<i64>,
    stealth_check: Option<i64>,
    damage: Option<WealthDice>,
    damage_type: Option<String>,
    fixed_damage: Option<serde_json::Value>,
    properties: Option<Vec<Property>>,
    attack_type: Option<i64>,
    category_id: Option<i64>,
    range: Option<i64>,
    long_range: Option<i64>,
    is_monk_weapon: bool,
    level_infusion_granted: Option<i64>,
    sources: Vec<Source>,
    armor_type_id: Option<i64>,
    gear_type_id: Option<i64>,
    grouped_id: Option<i64>,
    can_be_added_to_inventory: bool,
    is_container: bool,
    is_custom_item: bool,
    is_legacy: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FilterType {
    Armor,
    #[serde(rename = "Other Gear")]
    OtherGear,
    Weapon,
    #[serde(rename = "Wondrous item")]
    WondrousItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    id: i64,
    name: String,
    description: String,
    notes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    #[serde(rename = "Very Rare")]
    VeryRare
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryLimitedUse {
    max_uses: i64,
    number_used: i64,
    reset_type: String,
    reset_type_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modifiers {
    pub race: Vec<ItemElement>,
    pub class: Vec<ItemElement>,
    pub background: Vec<ItemElement>,
    pub item: Vec<ItemElement>,
    pub feat: Vec<Option<serde_json::Value>>,
    pub condition: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notes {
    allies: Option<serde_json::Value>,
    personal_possessions: String,
    other_holdings: Option<serde_json::Value>,
    organizations: Option<serde_json::Value>,
    enemies: Option<serde_json::Value>,
    backstory: String,
    other_notes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PactMagic {
    pub level: i64,
    pub used: i64,
    pub available: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    use_homebrew_content: bool,
    progression_type: i64,
    encumbrance_type: i64,
    ignore_coin_weight: bool,
    hit_point_type: i64,
    show_unarmed_strike: bool,
    show_scaled_spells: bool,
    primary_sense: i64,
    primary_movement: i64,
    privacy_type: i64,
    sharing_type: i64,
    ability_score_display_type: i64,
    enforce_feat_rules: bool,
    enforce_multiclass_rules: bool,
    enable_optional_class_features: bool,
    enable_optional_origins: bool,
    enable_dark_mode: bool,
    enable_container_currency: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    is_sub_race: bool,
    pub base_race_name: String,
    entity_race_id: i64,
    entity_race_type_id: i64,
    definition_key: String,
    full_name: String,
    base_race_id: i64,
    base_race_type_id: i64,
    description: String,
    avatar_url: Option<serde_json::Value>,
    large_avatar_url: Option<serde_json::Value>,
    portrait_avatar_url: String,
    more_details_url: String,
    is_homebrew: bool,
    is_legacy: bool,
    group_ids: Vec<i64>,
    #[serde(rename = "type")]
    race_type: i64,
    supports_subrace: Option<serde_json::Value>,
    sub_race_short_name: String,
    base_name: String,
    racial_traits: Vec<RacialTrait>,
    weight_speeds: WeightSpeeds,
    feat_ids: Vec<Option<serde_json::Value>>,
    size: Option<serde_json::Value>,
    size_id: i64,
    sources: Vec<Source>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RacialTrait {
    definition: RacialTraitDefinition,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RacialTraitDefinition {
    id: i64,
    definition_key: String,
    entity_type_id: i64,
    display_order: i64,
    name: String,
    description: String,
    snippet: Option<String>,
    hide_in_builder: bool,
    hide_in_sheet: bool,
    activation: Option<serde_json::Value>,
    source_id: i64,
    source_page_number: i64,
    creature_rules: Vec<Option<serde_json::Value>>,
    spell_list_ids: Vec<Option<serde_json::Value>>,
    feature_type: i64,
    sources: Vec<Source>,
    affected_feature_definition_keys: Vec<Option<serde_json::Value>>,
    is_called_out: bool,
    entity_type: String,
    #[serde(rename = "entityID")]
    entity_id: String,
    entity_race_id: i64,
    entity_race_type_id: i64,
    display_configuration: DisplayConfiguration,
    required_level: Option<serde_json::Value>,
    categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    id: i64,
    entity_type_id: i64,
    entity_id: i64,
    definition_key: String,
    entity_tag_id: i64,
    tag_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct DisplayConfiguration {
    racialtrait: i64,
    language: i64,
    abilityscore: i64,
    classfeature: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightSpeeds {
    normal: Normal,
    encumbered: Option<serde_json::Value>,
    heavily_encumbered: Option<serde_json::Value>,
    push_drag_lift: Option<serde_json::Value>,
    #[serde(rename = "override")]
    weight_speeds_override: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Normal {
    walk: i64,
    fly: i64,
    burrow: i64,
    swim: i64,
    climb: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Traits {
    personality_traits: Option<serde_json::Value>,
    ideals: Option<serde_json::Value>,
    bonds: Option<serde_json::Value>,
    flaws: Option<serde_json::Value>,
    appearance: Option<serde_json::Value>,
}

}
