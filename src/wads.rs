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

pub fn get_wad_by_checksum(crc: u32) -> &'static Wad<'static> {
    return match crc {
        0xCBADB0E9 => &S1_JULY_PROTO,
        0x9E352E51 => &S1_JUNE_PROTO,
        _ => &UNKNOWN_WAD
    }
}