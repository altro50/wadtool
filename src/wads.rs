#[derive(Debug)]
pub struct Wad<'a> {
    pub name: &'a str,
    pub filenames: &'static [&'static str],
    pub documented: bool
}

static UNKNOWN_WAD: Wad = Wad {
    name: "Unknown wad file",
    filenames: &[],
    documented: false
};

static S1_JUNE_PROTO: Wad = Wad {
    name: "Spyro the Dragon (Jun 15 Prototype)",
    filenames: &[
        "intro1.wad",
        "intro2.wad",
        "unk1.wad",
        "unk2.wad",
        "level_0_artisans_home_code.ovl",
        "level_0_artisans_home_data.wad",
        "level_1_artisans_stone_hill_code.ovl",
        "level_1_artisans_stone_hill_data.wad",
        "level_2_artisans_dark_hollow_code.ovl",
        "level_2_artisans_dark_hollow_data.wad",
        "level_3_artisans_town_square_code.ovl",
        "level_3_artisans_town_square_data.wad",
        "level_4_artisans_boss_code.ovl",
        "level_4_artisans_boss_data.wad",
        "level_5_artisans_flying_code.ovl",
        "level_5_artisans_flying_data.wad",
        "level_6_peace_keepers_home_code.ovl",
        "level_6_peace_keepers_home_data.wad",
        "level_7_peace_keepers_dry_canyon_code.ovl",
        "level_7_peace_keepers_dry_canyon_data.wad",
        "level_8_peace_keepers_cliff_town_code.ovl",
        "level_8_peace_keepers_cliff_town_data.wad",
        "level_9_peace_keepers_ice_cavern_code.ovl",
        "level_9_peace_keepers_ice_cavern_data.wad",
        "level_10_peace_keepers_boss_code.ovl",
        "level_10_peace_keepers_boss_data.wad",
        "level_11_peace_keepers_flying_code.ovl",
        "level_11_peace_keepers_flying_data.wad",
        "level_12_magic_crafters_home_code.ovl",
        "level_12_magic_crafters_home_data.wad",
        "level_13_magic_crafters_alpine_cliffs_code.ovl",
        "level_13_magic_crafters_alpine_cliffs_data.wad",
        "level_14_magic_crafters_high_caves_code.ovl",
        "level_14_magic_crafters_high_caves_data.wad",
        "level_15_magic_crafters_wizard_peak_code.ovl",
        "level_15_magic_crafters_wizard_peak_data.wad",
        "level_16_magic_crafters_boss_code.ovl",
        "level_16_magic_crafters_boss_data.wad",
        "level_17_magic_crafters_flying_code.ovl",
        "level_17_magic_crafters_flying_data.wad",
        "level_18_beast_makers_home_code.ovl",
        "level_18_beast_makers_home_data.wad",
        "level_19_beast_makers_terrace_village_code.ovl",
        "level_19_beast_makers_terrace_village_data.wad",
        "level_20_beast_makers_misty_bog_code.ovl",
        "level_20_beast_makers_misty_bog_data.wad",
        "level_21_beast_makers_tree_tops_code.ovl",
        "level_21_beast_makers_tree_tops_data.wad",
        "level_22_beast_makers_boss_code.ovl",
        "level_22_beast_makers_boss_data.wad",
        "level_23_beast_makers_flying_code.ovl",
        "level_23_beast_makers_flying_data.wad",
        "level_24_dream_weavers_home_code.ovl",
        "level_24_dream_weavers_home_data.wad",
        "level_25_dream_weavers_dark_passage_code.ovl",
        "level_25_dream_weavers_dark_passage_data.wad",
        "level_26_dream_weavers_lofty_castle_code.ovl",
        "level_26_dream_weavers_lofty_castle_data.wad",
        "level_27_dream_weavers_haunted_towers_code.ovl",
        "level_27_dream_weavers_haunted_towers_data.wad",
        "level_28_dream_weavers_boss_code.ovl",
        "level_28_dream_weavers_boss_data.wad",
        "level_29_dream_weavers_flying_code.ovl",
        "level_29_dream_weavers_flying_data.wad",
        "level_30_machinists_gnorc_cove_code.ovl",
        "level_30_machinists_gnorc_cove_data.wad",
        "level_31_machinists_the_end_code.ovl",
        "level_31_machinists_the_end_data.wad",
        "level_32_machinists_bunchacrap_code.ovl",
        "level_32_machinists_bunchacrap_data.wad",
        "level_33_machinists_gnasty_place_code.ovl",
        "level_33_machinists_gnasty_place_data.wad"
    ],
    documented: true
};

static S1_JULY_PROTO: Wad = Wad {
    name: "Spyro the Dragon (Jul 19 Prototype)",
    filenames: &[
        "intro1.wad",
        "intro2.wad",
        "unk1.wad",
        "unk2.wad",
        "level_0_artisans_home_code.ovl",
        "level_0_artisans_home_data.wad",
        "level_1_artisans_stone_hill_code.ovl",
        "level_1_artisans_stone_hill_data.wad",
        "level_2_artisans_dark_hollow_code.ovl",
        "level_2_artisans_dark_hollow_data.wad",
        "level_3_artisans_town_square_code.ovl",
        "level_3_artisans_town_square_data.wad",
        "level_4_artisans_toasty_code.ovl",
        "level_4_artisans_toasty_data.wad",
        "level_5_artisans_sunny_flight_code.ovl",
        "level_5_artisans_sunny_flight_data.wad",
        "level_6_peace_keepers_home_code.ovl",
        "level_6_peace_keepers_home_data.wad",
        "level_7_peace_keepers_dry_canyon_code.ovl",
        "level_7_peace_keepers_dry_canyon_data.wad",
        "level_8_peace_keepers_cliff_town_code.ovl",
        "level_8_peace_keepers_cliff_town_data.wad",
        "level_9_peace_keepers_ice_cavern_code.ovl",
        "level_9_peace_keepers_ice_cavern_data.wad",
        "level_10_peace_keepers_doctor_shemp_code.ovl",
        "level_10_peace_keepers_doctor_shemp_data.wad",
        "level_11_peace_keepers_night_flight_code.ovl",
        "level_11_peace_keepers_night_flight_data.wad",
        "level_12_magic_crafters_home_code.ovl",
        "level_12_magic_crafters_home_data.wad",
        "level_13_magic_crafters_alpine_ridge_code.ovl",
        "level_13_magic_crafters_alpine_ridge_data.wad",
        "level_14_magic_crafters_high_caves_code.ovl",
        "level_14_magic_crafters_high_caves_data.wad",
        "level_15_magic_crafters_wizard_peak_code.ovl",
        "level_15_magic_crafters_wizard_peak_data.wad",
        "level_16_magic_crafters_blowhard_code.ovl",
        "level_16_magic_crafters_blowhard_data.wad",
        "level_17_magic_crafters_crystal_flight_code.ovl",
        "level_17_magic_crafters_crystal_flight_data.wad",
        "level_18_beast_makers_home_code.ovl",
        "level_18_beast_makers_home_data.wad",
        "level_19_beast_makers_terrace_village_code.ovl",
        "level_19_beast_makers_terrace_village_data.wad",
        "level_20_beast_makers_misty_bog_code.ovl",
        "level_20_beast_makers_misty_bog_data.wad",
        "level_21_beast_makers_tree_tops_code.ovl",
        "level_21_beast_makers_tree_tops_data.wad",
        "level_22_beast_makers_metalhead_code.ovl",
        "level_22_beast_makers_metalhead_data.wad",
        "level_23_beast_makers_wild_flight_code.ovl",
        "level_23_beast_makers_wild_flight_data.wad",
        "level_24_dream_weavers_home_code.ovl",
        "level_24_dream_weavers_home_data.wad",
        "level_25_dream_weavers_dark_passage_code.ovl",
        "level_25_dream_weavers_dark_passage_data.wad",
        "level_26_dream_weavers_lofty_castle_code.ovl",
        "level_26_dream_weavers_lofty_castle_data.wad",
        "level_27_dream_weavers_haunted_towers_code.ovl",
        "level_27_dream_weavers_haunted_towers_data.wad",
        "level_28_dream_weavers_jacques_code.ovl",
        "level_28_dream_weavers_jacques_data.wad",
        "level_29_dream_weavers_icy_flight_code.ovl",
        "level_29_dream_weavers_icy_flight_data.wad",
        "level_30_gnastys_world_gnorc_gnexus_code.ovl",
        "level_30_gnastys_world_gnorc_gnexus_data.wad",
        "level_31_gnastys_world_gnorc_cove_code.ovl",
        "level_31_gnastys_world_gnorc_cove_data.wad",
        "level_32_gnastys_world_twilight_harbor_code.ovl",
        "level_32_gnastys_world_twilight_harbor_data.wad",
        "level_33_gnastys_world_gnasty_place_code.ovl",
        "level_33_gnastys_world_gnasty_place_data.wad",
        "level_34_gnastys_world_gnasty_booty_code.ovl",
        "level_34_gnastys_world_gnasty_booty_data.wad"
    ],
    documented: true
};

static S1_NTSC_FINAL: Wad = Wad {
    name: "Spyro the Dragon (NTSC Final)",
    filenames: &[
        // Unknown, loaded on game boot, might be graphics
        "0.wad",
        "1.wad",

        // Code and data for the title screen
        "titlescreen_code.ovl",
        "titlescreen_data.wad",

        // Intro and outro cutscenes
        "intro_data.wad",
        "outro1_data.wad",
        "outro2_data.wad",

        // Not sure about these yet
        "8.wad",
        "9.wad",

        // Level overlay and data wads
        // Overlays are loaded to $8007aa38 in this version
        "level_0_artisans_home_code.ovl",
        "level_0_artisans_home_data.wad",
        "level_1_artisans_stone_hill_code.ovl",
        "level_1_artisans_stone_hill_data.wad",
        "level_2_artisans_dark_hollow_code.ovl",
        "level_2_artisans_dark_hollow_data.wad",
        "level_3_artisans_town_square_code.ovl",
        "level_3_artisans_town_square_data.wad",
        "level_4_artisans_toasty_code.ovl",
        "level_4_artisans_toasty_data.wad",
        "level_5_artisans_sunny_flight_code.ovl",
        "level_5_artisans_sunny_flight_data.wad",
        "level_6_peace_keepers_home_code.ovl",
        "level_6_peace_keepers_home_data.wad",
        "level_7_peace_keepers_dry_canyon_code.ovl",
        "level_7_peace_keepers_dry_canyon_data.wad",
        "level_8_peace_keepers_cliff_town_code.ovl",
        "level_8_peace_keepers_cliff_town_data.wad",
        "level_9_peace_keepers_ice_cavern_code.ovl",
        "level_9_peace_keepers_ice_cavern_data.wad",
        "level_10_peace_keepers_doctor_shemp_code.ovl",
        "level_10_peace_keepers_doctor_shemp_data.wad",
        "level_11_peace_keepers_night_flight_code.ovl",
        "level_11_peace_keepers_night_flight_data.wad",
        "level_12_magic_crafters_home_code.ovl",
        "level_12_magic_crafters_home_data.wad",
        "level_13_magic_crafters_alpine_ridge_code.ovl",
        "level_13_magic_crafters_alpine_ridge_data.wad",
        "level_14_magic_crafters_high_caves_code.ovl",
        "level_14_magic_crafters_high_caves_data.wad",
        "level_15_magic_crafters_wizard_peak_code.ovl",
        "level_15_magic_crafters_wizard_peak_data.wad",
        "level_16_magic_crafters_blowhard_code.ovl",
        "level_16_magic_crafters_blowhard_data.wad",
        "level_17_magic_crafters_crystal_flight_code.ovl",
        "level_17_magic_crafters_crystal_flight_data.wad",
        "level_18_beast_makers_home_code.ovl",
        "level_18_beast_makers_home_data.wad",
        "level_19_beast_makers_terrace_village_code.ovl",
        "level_19_beast_makers_terrace_village_data.wad",
        "level_20_beast_makers_misty_bog_code.ovl",
        "level_20_beast_makers_misty_bog_data.wad",
        "level_21_beast_makers_tree_tops_code.ovl",
        "level_21_beast_makers_tree_tops_data.wad",
        "level_22_beast_makers_metalhead_code.ovl",
        "level_22_beast_makers_metalhead_data.wad",
        "level_23_beast_makers_wild_flight_code.ovl",
        "level_23_beast_makers_wild_flight_data.wad",
        "level_24_dream_weavers_home_code.ovl",
        "level_24_dream_weavers_home_data.wad",
        "level_25_dream_weavers_dark_passage_code.ovl",
        "level_25_dream_weavers_dark_passage_data.wad",
        "level_26_dream_weavers_lofty_castle_code.ovl",
        "level_26_dream_weavers_lofty_castle_data.wad",
        "level_27_dream_weavers_haunted_towers_code.ovl",
        "level_27_dream_weavers_haunted_towers_data.wad",
        "level_28_dream_weavers_jacques_code.ovl",
        "level_28_dream_weavers_jacques_data.wad",
        "level_29_dream_weavers_icy_flight_code.ovl",
        "level_29_dream_weavers_icy_flight_data.wad",
        "level_30_gnastys_world_gnorc_gnexus_code.ovl",
        "level_30_gnastys_world_gnorc_gnexus_data.wad",
        "level_31_gnastys_world_gnorc_cove_code.ovl",
        "level_31_gnastys_world_gnorc_cove_data.wad",
        "level_32_gnastys_world_twilight_harbor_code.ovl",
        "level_32_gnastys_world_twilight_harbor_data.wad",
        "level_33_gnastys_world_gnasty_gnorc_code.ovl",
        "level_33_gnastys_world_gnasty_gnorc_data.wad",
        "level_34_gnastys_world_gnastys_loot_code.ovl",
        "level_34_gnastys_world_gnastys_loot_data.wad",

        // Two empty files of unknown purpose
        "empty1.wad",
        "empty2.wad",

        // Overlay for the credits related stuff
        "credits.ovl",

        // Beating gnasty credits
        "credits1_data_0.wad",
        "credits1_data_1.wad",
        "credits1_data_2.wad",
        "credits1_data_3.wad",
        "credits1_data_4.wad",
        "credits1_data_5.wad",
        "credits1_data_6.wad",
        "credits1_data_7.wad",
        "credits1_data_8.wad",
        "credits1_data_9.wad",

        // Game complete credits
        "credits2_data_0.wad",
        "credits2_data_1.wad",
        "credits2_data_2.wad",
        "credits2_data_3.wad",
        "credits2_data_4.wad",
        "credits2_data_5.wad",
        "credits2_data_6.wad",
        "credits2_data_7.wad",
        "credits2_data_8.wad",
        "credits2_data_9.wad",
    ],
    documented: true
};



static S3_NTSC_1_0: Wad = Wad {
    name: "Spyro: Year of the Dragon (NTSC 1.0)",
    filenames: &[
        "0.wad",
        "1.wad",
        "2.wad",
        "universal_logo.wad",
        "4.wad",
        "insomniac_logo.wad",
        "titlescreen_level.wad",
        "7.wad",
        "8.wad",

        "cutscene1_data.wad",
        "cutscene1_code.ovl",
        "cutscene1_unknown.wad",

        "cutscene2_data.wad",
        "cutscene2_code.ovl",
        "cutscene2_unknown.wad",

        "cutscene3_data.wad",
        "cutscene3_code.ovl",
        "cutscene3_unknown.wad",

        "cutscene4_data.wad",
        "cutscene4_code.ovl",
        "cutscene4_unknown.wad",

        "cutscene5_data.wad",
        "cutscene5_code.ovl",
        "cutscene5_unknown.wad",

        "cutscene6_data.wad",
        "cutscene6_code.ovl",
        "cutscene6_unknown.wad",

        "cutscene7_data.wad",
        "cutscene7_code.ovl",
        "cutscene7_unknown.wad",

        "cutscene8_data.wad",
        "cutscene8_code.ovl",
        "cutscene8_unknown.wad",

        "cutscene9_data.wad",
        "cutscene9_code.ovl",
        "cutscene9_unknown.wad",

        "cutscene10_data.wad",
        "cutscene10_code.ovl",
        "cutscene10_unknown.wad",

        "cutscene11_data.wad",
        "cutscene11_code.ovl",
        "cutscene11_unknown.wad",

        "cutscene12_data.wad",
        "cutscene12_code.ovl",
        "cutscene12_unknown.wad",

        "cutscene13_data.wad",
        "cutscene13_code.ovl",
        "cutscene13_unknown.wad",

        "cutscene14_data.wad",
        "cutscene14_code.ovl",
        "cutscene14_unknown.wad",

        "cutscene15_data.wad",
        "cutscene15_code.ovl",
        "cutscene15_unknown.wad",

        "cutscene16_data.wad",
        "cutscene16_code.ovl",
        "cutscene16_unknown.wad",

        "cutscene17_data.wad",
        "cutscene17_code.ovl",
        "cutscene17_unknown.wad",

        "cutscene18_data.wad",
        "cutscene18_code.ovl",
        "cutscene18_unknown.wad",

        "cutscene19_data.wad",
        "cutscene19_code.ovl",
        "cutscene19_unknown.wad",

        "cutscene20_data.wad",
        "cutscene20_code.ovl",
        "cutscene20_unknown.wad",

        // Loaded in during the level transition, and in several other places.
        "common_code.ovl", 

        // Loading screen images
        "sunrise_springs_loading.bgr555",
        "midday_gardens_loading.bgr555",
        "evening_lake_loading.bgr555",
        "midnight_mountain_loading.bgr555",
        "crawdad_loading.bgr555",
        "ourherotakesarest.bgr555",
        "cutscene1_loading.bgr555", // Intro 1
        "cutscene2_loading.bgr555", // Second warning
        "cutscene3_loading.bgr555",
        "cutscene4_loading.bgr555",
        "cutscene5_loading.bgr555",
        "cutscene6_loading.bgr555",
        "cutscene7_loading.bgr555",
        "cutscene8_loading.bgr555",
        "cutscene9_loading.bgr555", // Outro interview scene
        "cutscene10_loading.bgr555", // Outro Agent 9 scene
        "cutscene11_loading.bgr555", // Outro Sheila scene
        "cutscene12_loading.bgr555", // Empty, removed cutscene?
        "cutscene13_loading.bgr555", // Outro FF scene
        "cutscene14_loading.bgr555", // Outro babies scene
        "cutscene15_loading.bgr555", // Intro 2
        "cutscene16_loading.bgr555", // Intro 3
        "cutscene17_loading.bgr555", // Sheila Moneybags scene
        "cutscene18_loading.bgr555", // Sgt. Byrd Moneybags scene
        "cutscene19_loading.bgr555", // Bentley Moneybags scene
        "cutscene20_loading.bgr555", // Agent 9 Moneybags scene

        // All of the skyboxes used in the level transition
        "level_transition_skyboxes.wad",

        // Level data
        // I should note here that the level overlays in Spyro 3 are ""encrypted""
        // The chipher is very easy, starting from $1000 you XOR every value (as a DWORD) with the address specified at $4 (the address itself, not dereferenced)
        // Decryption stops at the value specified at $4.

        // Overlays are loaded at $800742d0 in this build
        
        "level_0_sunrise_spring_home_data.wad",
        "level_0_sunrise_spring_home_code.ovl",

        "level_1_sunny_villa_data.wad",
        "level_1_sunny_villa_code.ovl",

        "level_2_cloud_spires_data.wad",
        "level_2_cloud_spires_code.ovl",

        "level_3_molten_crater_data.wad",
        "level_3_molten_crater_code.ovl",
        
        "level_4_seashell_shore_data.wad",
        "level_4_seashell_shore_code.ovl",

        "level_5_mushroom_speedway_data.wad",
        "level_5_mushroom_speedway_code.ovl",

        "level_6_sheila's_alp_data.wad",
        "level_6_sheila's_alp_code.ovl",

        "level_7_buzz's_dungeon_data.wad",
        "level_7_buzz's_dungeon_code.ovl",

        "level_8_crawdad_farm_data.wad",
        "level_8_crawdad_farm_code.ovl",

        "level_9_midday_garden_home_data.wad",
        "level_9_midday_garden_home_code.ovl",

        "level_10_icy_peak_data.wad",
        "level_10_icy_peak_code.ovl",

        "level_11_enchanted_towers_data.wad",
        "level_11_enchanted_towers_code.ovl",

        "level_12_spooky_swamp_data.wad",
        "level_12_spooky_swamp_code.ovl",

        "level_13_bamboo_terrace_data.wad",
        "level_13_bamboo_terrace_code.ovl",

        "level_14_country_speedway_data.wad",
        "level_14_country_speedway_code.ovl",

        "level_15_sgt._byrd's_base_data.wad",
        "level_15_sgt._byrd's_base_code.ovl",

        "level_16_spike's_arena_data.wad",
        "level_16_spike's_arena_code.ovl",

        "level_17_spider_town_data.wad",
        "level_17_spider_town_code.ovl",

        "level_18_evening_lake_home_data.wad",
        "level_18_evening_lake_home_code.ovl",

        "level_19_frozen_altars_data.wad",
        "level_19_frozen_altars_code.ovl",

        "level_20_lost_fleet_data.wad",
        "level_20_lost_fleet_code.ovl",

        "level_21_fireworks_factory_data.wad",
        "level_21_fireworks_factory_code.ovl",

        "level_22_charmed_ridge_data.wad",
        "level_22_charmed_ridge_code.ovl",

        "level_23_honey_speedway_data.wad",
        "level_23_honey_speedway_code.ovl",

        "level_24_bentley's_outpost_data.wad",
        "level_24_bentley's_outpost_code.ovl",

        "level_25_scorch's_pit_data_data.wad",
        "level_25_scorch's_pit_code_code.ovl",

        "level_26_starfish_reef_data_data.wad",
        "level_26_starfish_reef_code_code.ovl",

        "level_27_midnight_mountain_home_data.wad",
        "level_27_midnight_mountain_home_code.ovl",

        "level_28_crystal_islands_data.wad",
        "level_28_crystal_islands_code.ovl",

        "level_29_desert_ruins_data.wad",
        "level_29_desert_ruins_code.ovl",

        "level_30_haunted_tomb_data.wad",
        "level_30_haunted_tomb_code.ovl",

        "level_31_dino_mines_data.wad",
        "level_31_dino_mines_code.ovl",

        "level_32_harbor_speedway_data.wad",
        "level_32_harbor_speedway_code.ovl",

        "level_33_agent_9's_lab_data.wad",
        "level_33_agent_9's_lab_code.ovl",

        "level_34_sorceress'_lair_data.wad",
        "level_34_sorceress'_lair_code.ovl",

        "level_35_bugbot_factory_data.wad",
        "level_35_bugbot_factory_code.ovl",

        "level_36_super_bonus_round_data.wad",
        "level_36_super_bonus_round_code.ovl",

        // Bunch of empty files, they're mentioned in the Atlas overlay, however.
        "unknown_empty1.wad",
        "unknown_empty2.wad",
        "unknown_empty3.wad",
        "unknown_empty4.wad",
        "unknown_empty5.wad",
        "unknown_empty6.wad",

        "gameover_screen.bgr555",

        "atlas_code.ovl",
        "options_menu_code.ovl",
        
        // Atlas level graphics
        "atlas_graphics.wad",

        // Images used in the epilogue
        "epilogue_graphics.wad",

        // Credits stuff
        "credits_code.ovl",
        "credits_data_0.wad", // Sheila's Alp
        "credits_data_1.wad", // Seashell Shore
        "credits_data_2.wad", // Icy Peak
        "credits_data_3.wad", // Spooky Swamp
        "credits_data_4.wad", // Lost Fleet sublevel
        "credits_data_5.wad", // Fireworks Factory sublevel
        "credits_data_6.wad", // Desert Ruins
        "credits_data_7.wad", // Dino Mines
        "credits_data_8.wad", // Charmed Ridge sublevel
        "credits_data_9.wad", // Haunted Tomb
        "credits_data_10.wad", // Cloud Spires
        "credits_data_11.wad" // Bamboo Terrace sublevel
    ],
    documented: true
};

pub fn get_wad_by_checksum(crc: u32) -> &'static Wad<'static> {
    return match crc {
        0xCBADB0E9 => &S1_JULY_PROTO,
        0x9E352E51 => &S1_JUNE_PROTO,
        0xEEEFDD83 => &S1_NTSC_FINAL,
        0x5886EC7D => &S3_NTSC_1_0,
        _ => &UNKNOWN_WAD
    }
}