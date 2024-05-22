use super::*;
use crate::game::globals;

pub async fn on_start_cocoon_stage_cs_req(
    session: &PlayerSession,
    body: &StartCocoonStageCsReq,
) -> Result<()> {
    let player_info = session.player_info();

    let rsp = StartCocoonStageScRsp {
        retcode: 0,
        prop_entity_id: body.prop_entity_id,
        cocoon_id: body.cocoon_id,
        wave: body.wave,
        battle_info: Some(SceneBattleInfo {
            stage_id: 201012311,
            logic_random_seed: 4444,
            battle_id: 1,
            battle_avatar_list: player_info
                .lineup
                .avatar_list
                .iter()
                .map(|avatar| BattleAvatar {
                    index: avatar.slot,
                    id: avatar.id,
                    level: match avatar.id {
                        1310 => 90000, // Firefly
                        _ => 80 // others
                    },
                    promotion: 6,
                    rank: match avatar.id {
                        1202 => 6, // Tingyun
                        1106 => 6, // Pela
                        8005 => 6, // Harmony TB
                        1310 => 6, // Firefly
                        1314 => 6, //Jade
                        _ => 0 // others
                    },
                    skilltree_list: [1, 2, 3, 4, 7, 101, 102, 103, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210]
                    .iter()
                    .map(|end| {
                        let mut level = 1;
                        if *end == 1 {
                            level = 6;
                        } else if *end <= 4 {
                            level = 10;
                        }
                        if *end > 4 {
                            level = 1;
                        }
                        AvatarSkillTree {
                            point_id: avatar.id * 1000 + *end,
                            level,
                        }
                    })
                    .collect(),
                    hp: 10000,
                    avatar_type: 3,
                    sp: Some(AmountInfo {
                        cur_amount: 10000,
                        max_amount: 10000,
                    }),
                    relic_list: match avatar.id {
                        // Firefly
                        1310 => vec![ // 4pc- Thief of Shooting Meteor- break effect set and 2PC salsatto Set
                            BattleRelic {
                                id: 61191,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            },
							
                            BattleRelic {
                                id: 61192,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                     RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61193,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61194,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63165,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                                     RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63166,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 4,
                                        step: 8,
                                    },
									 RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],
						
                        // Robin
                        1309 => vec![ // 2PC musketor and 2pc prisoner & 2PC- Space Sealing Station
                            BattleRelic {
                                id: 61011,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61022,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61163,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61164,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63015,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63016,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],

                        // Jade 
                        1314 => vec![ // 2PC musketor and 2pc prisoner & 2PC- Space Sealing Station
                            BattleRelic {
                                id: 61201,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61202,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61203,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61204,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63015,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63016,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],
                        // Ruan Mei
                        1303 => vec![ // 4 PC Watchmaker, Master of Dream Machinations & 2pc- Sprightly Vonwacq
                            BattleRelic {
                                id: 61181,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61182,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61183,
                                level: 15,
                                main_affix_id: 3,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61184,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63085,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63086,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],

                        // Huo Huo
                        1217 => vec![ // 4pc messenger & 2pc-Sprightly Vonwacq
                            BattleRelic {
                                id: 61141,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61142,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61143,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61144,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63085,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63086,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],
						
						// Gallagher
                        1301 => vec![ // 2pc messenger 2pc thief & 2pc-Fleetless
                            BattleRelic {
                                id: 61011,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61012,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61113,
                                level: 15,
                                main_affix_id: 6,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61114,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63025,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63026,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],

                        // Sparkle
                        1306 => vec![ // 4PC-Messenger Traversing Hackerspace & 2 PC-Penacony, Land of the Dreams + 2 PC Prisoner in Deep Confinement
                            BattleRelic {
                                id: 61141,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61141,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61143,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61144,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63125,
                                level: 15,
                                main_affix_id: 3,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63126,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],

                        // Tingyun
                        1202 => vec![ // 2PC Musketeer of Wild Wheat + ??? & 2 PC- Penacony, Land of the Dreams
                            BattleRelic {
                                id: 61021,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61022,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61163,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61164,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63125,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63126,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],

                        // Aventurine
                        1304 => vec![ // 4PC-Knight of Purity Palace & 2PC- Belobog of the Architects
                            BattleRelic {
                                id: 61031,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61032,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61033,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61034,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63045,
                                level: 15,
                                main_affix_id: 3,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63046,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],

                        // Dr. Ratio
                        1305 => vec![ // 4PC- Pioneer Diver of Dead Waters & 2PC- Izumo Gensei and Takama Divine Realm
                            BattleRelic {
                                id: 61171,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61172,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61173,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61174,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63145,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63146,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],

                        // Topaz & Numby
                        1112 => vec![ // 4PC- Pioneer Diver of Dead Waters & 2PC- Izumo Gensei and Takama Divine Realm
                            BattleRelic {
                                id: 61171,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61172,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61173,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61174,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63145,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63146,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],

                        // Acheron
                        1308 => vec![ // 4PC- Pioneer Diver of Dead Waters & 2PC- Izumo Gensei and Takama Divine Realm
                            BattleRelic {
                                id: 61171,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61172,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61173,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61174,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63145,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63146,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 4,
                                        step: 8,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],

                        // Pela
                        1106 => vec![ // 4PC- Eagle of Twilight Line & 2 PC- Sprightly Vonwacq
                            BattleRelic {
                                id: 61101,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61102,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61103,
                                level: 15,
                                main_affix_id: 3,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61104,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63085,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63086,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],

                        // Black Swan
                        1307 => vec![ // 4PC- Prisoner in Deep Confinement & 2PC- Pan-Cosmic Commercial Enterprise
                            BattleRelic {
                                id: 61161,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61162,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61163,
                                level: 15,
                                main_affix_id: 7,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61164,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63035,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 63036,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],
                        8005 => vec![//Harmony TB 2pc watch maker 2pc thiefs of shooting stars 2pc talia
                            BattleRelic {
                                id: 61181,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },
                            BattleRelic {
                                id: 61182,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },
                            BattleRelic {
                                id: 61183,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 5,
                                        step: 10,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },
                            BattleRelic {
                                id: 61184,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 5,
                                        step: 10,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },
                            BattleRelic {
                                id: 63075,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },
                            BattleRelic {
                                id: 63076,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },
                        ],
                        // default - nothing
                        _ => vec![]
                    },
                    equipment_list: match avatar.id {
                        // Boothill
                        1315 => vec![ // LC- Cruising in the Stellar Sea- 24001- S5
                            BattleEquipment {
                                id: 24001,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],
						
						// Firefly
                        1310 => vec![ // LC- Whereabouts Should Dreams Rest- 23025- S5
                            BattleEquipment {
                                id: 23025,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],
                        //Jade
                        1314 => vec![ // LC- Whereabouts Should Dreams Rest- 23025- S5
                            BattleEquipment {
                                id: 23028,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],
						
						// Gallagher
                        1301 => vec![ // LC- Quid Pro Quo- 21021- S5
                            BattleEquipment {
                                id: 21021,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],

                        // Robin
                        1309 => vec![ // LC- For Tomorrow's Journey- S5- 22002
                            BattleEquipment {
                                id: 22002,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],

                        // Ruan Mei
                        1303 => vec![ // LC- Past Self in Mirror- S5- 21004
                            BattleEquipment {
                                id: 23019,
                                level: 80,
                                promotion: 6,
                                rank: 1
                            }
                        ],
                       // Harmony TB 
                        8005 => vec![ // LC- Memories of the Past- S5- 21004
                            BattleEquipment {
                                id: 21004,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],
                        // Huo Huo
                        1217 => vec![ // LC- Hey, Over Here- S5- 22001
                            BattleEquipment {
                                id: 22001,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],

                        // Sparkle
                        1306 => vec![ // LC- Past and Future- S5- 21025
                            BattleEquipment {
                                id: 21025,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],

                        // Tingyun
                        1202 => vec![ // LC- Meshing Cogs- S5- 20012
                            BattleEquipment {
                                id: 20012,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],

                        // Aventurine
                        1304 => vec![ // LC- Concert for Two- S5- 21043
                            BattleEquipment {
                                id: 21043,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],

                        // Dr. Ratio
                        1305 => vec![ // LC- 24001- Cruising in the Stellar Sea- S5
                            BattleEquipment {
                                id: 24001,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],

                        // Topaz & Numby
                        1112 => vec![ // LC- FINAL VICTOR- S5- 21037
                            BattleEquipment {
                                id: 21037,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],

                        //  Acheron
                        1308 => vec![ // LC- Boundless Choreo- S5- 21044
                            BattleEquipment {
                                id: 21044,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],

                        // Pela
                        1106 => vec![ // LC- Before the Tutorial Mission Starts- 22000
                            BattleEquipment {
                                id: 22000,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],

                        // Black Swan
                        1307 => vec![ // LC- Eyes of the Prey- S5- 21008
                            BattleEquipment {
                                id: 21008,
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],
						
						// default - nothing
                        _ => vec![]
                    },
                    ..Default::default()
                })
                .collect(),
            monster_wave_list: globals
                .monster_wave_list
                .iter()
                .map(|monster_list| SceneMonsterWave {
                    monster_list: monster_list
                        .iter()
                        .map(|id| SceneMonsterData {
                            monster_id: *id,
                            max_hp: 100000000, // Enemies Max HP
                            jjhfeikbakk: 1, //Enemies Level
                            ..Default::default()
                        })
                        .collect(),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }),
    };

    session.send(CMD_START_COCOON_STAGE_SC_RSP, rsp).await
}

pub async fn on_get_cur_battle_info_cs_req(
    session: &PlayerSession,
    _body: &GetCurBattleInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_CUR_BATTLE_INFO_SC_RSP,
            GetCurBattleInfoScRsp {
                battle_info: Some(SceneBattleInfo::default()),
                ffbpkghgmjm: Some(Fjojkdhlonn::default()),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_pve_battle_result_cs_req(
    session: &PlayerSession,
    body: &PveBattleResultCsReq,
) -> Result<()> {
    session
        .send(
            CMD_P_V_E_BATTLE_RESULT_SC_RSP,
            PveBattleResultScRsp {
                retcode: 0,
                end_status: body.end_status,
                battle_id: body.battle_id,
                ..Default::default()
            },
        )
        .await
}
