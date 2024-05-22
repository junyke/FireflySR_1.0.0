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
            /*set technique
                if (battleAvatarBuffs.ContainsKey(avatarId))
                {
                    var buff = new BattleBuff();
                    buff.Bbbejilfdhi = 0xffffffff;
                    buff.Eleplppdjbo = (uint)index;
                    buff.Level = 1;
                    buff.Id = 130301;
                    battleInfo.BuffLists.Add(buff);
                }*/
            battle_avatar_list: player_info
                .lineup
                .avatar_list
                .iter()
                .map(|avatar| BattleAvatar {
                    index: avatar.slot,
                    id: if avatar.id == 8001 {
                        player_info.hero_basic_type as u32
                    } else {
                        avatar.id
                    },
                    level: 80,
                    promotion: 6,
                    rank: match avatar.id {
                        1310 => 2, //Firefly
                            8006 => 6, // MC Harmony
                            1303 => 1, // Ruan Mei
                            1301 => 6, // Gallagher
                            _ => 6 // Others
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
                        cur_amount: 5000,
                        max_amount: 10000,
                    }),
                    relic_list: match avatar.id {

                        //FireFly 
                        1310 => vec![
                                BattleRelic {
                                    id: 61191,
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
                                    cnt: 3,
                                    step: 3,
                                    },
                                    RelicAffix {
                                    affix_id: 6,
                                    cnt: 1,
                                    step: 2,
                                    },
                                    RelicAffix {
                                    affix_id: 7,
                                    cnt: 4,
                                    step: 4,
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
                                affix_id: 6,
                                cnt: 2,
                                step: 2,
                                },
                                RelicAffix {
                                affix_id: 7,
                                cnt: 4,
                                step: 0,
                                },
                                RelicAffix {
                                affix_id: 9,
                                cnt: 2,
                                step: 2,
                                },
                                RelicAffix {
                                affix_id: 12,
                                cnt: 1,
                                step: 0,
                                },
                                ],
                            ..Default::default()      
                        },
                        BattleRelic {
                            id: 61193,
                            level: 15,
                            main_affix_id: 4,
                            sub_affix_list: vec![
                            RelicAffix {
                            affix_id: 7,
                            cnt: 1,
                            step: 1,
                            },
                            RelicAffix {
                            affix_id: 8,
                            cnt: 2,
                            step: 2,
                            },
                            RelicAffix {
                            affix_id: 11,
                            cnt: 2,
                            step: 2,
                            },
                            RelicAffix {
                            affix_id: 12,
                            cnt: 3,
                            step: 3,
                            },
                            ],
                        ..Default::default()      
                    },
                    BattleRelic {
                        id: 61194,
                        level: 15,
                        main_affix_id: 4,
                        sub_affix_list: vec![
                        RelicAffix {
                        affix_id: 5,
                        cnt: 2,
                        step: 4,
                        },
                        RelicAffix {
                        affix_id: 5,
                        cnt: 2,
                        step: 2,
                        },
                        RelicAffix {
                        affix_id: 8,
                        cnt: 2,
                        step: 0,
                        },
                        RelicAffix {
                        affix_id: 12,
                        cnt: 2,
                        step: 0,
                        },
                        ],
                    ..Default::default()      
                },
                BattleRelic {
                    id: 63165,
                    level: 15,
                    main_affix_id: 2,
                    sub_affix_list: vec![
                    RelicAffix {
                    affix_id: 4,
                    cnt: 1,
                    step: 1,
                    },
                    RelicAffix {
                    affix_id: 6,
                    cnt: 2,
                    step: 2,
                    },
                    RelicAffix {
                    affix_id: 11,
                    cnt: 1,
                    step: 1,
                    },
                    RelicAffix {
                    affix_id: 12,
                    cnt: 3,
                    step: 3,
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
                cnt: 2,
                step: 0,
                },
                RelicAffix {
                affix_id: 5,
                cnt: 3,
                step: 3,
                },
                RelicAffix {
                affix_id: 7,
                cnt: 1,
                step: 1,
                },
                RelicAffix {
                affix_id: 8,
                cnt: 2,
                step: 0,
                },
                ],
            ..Default::default()      
        },
            
                        ], 
                        
                        


                        //Gallagher build 
                        1301 => vec![
                                BattleRelic {
                                    id: 61181,
                                    level: 15,
                                    main_affix_id: 1,
                                    sub_affix_list: vec![
                                    RelicAffix {
                                    affix_id: 4,
                                    cnt: 2,
                                    step: 1,
                                    },
                                    RelicAffix {
                                    affix_id: 7,
                                    cnt: 3,
                                    step: 3,
                                    },
                                    RelicAffix {
                                    affix_id: 11,
                                    cnt: 1,
                                    step: 1,
                                    },
                                    RelicAffix {
                                    affix_id: 12,
                                    cnt: 2,
                                    step: 0,
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
                                affix_id: 4,
                                cnt: 1,
                                step: 1,
                                },
                                RelicAffix {
                                affix_id: 5,
                                cnt: 1,
                                step: 0,
                                },
                                RelicAffix {
                                affix_id: 7,
                                cnt: 3,
                                step: 3,
                                },
                                RelicAffix {
                                affix_id: 12,
                                cnt: 3,
                                step: 3,
                                },
                                ],
                            ..Default::default()      
                        },
                        BattleRelic {
                            id: 61143,
                            level: 15,
                            main_affix_id: 6,
                            sub_affix_list: vec![
                            RelicAffix {
                            affix_id: 2,
                            cnt: 3,
                            step: 2,
                            },
                            RelicAffix {
                            affix_id: 4,
                            cnt: 1,
                            step: 2,
                            },
                            RelicAffix {
                            affix_id: 11,
                            cnt: 1,
                            step: 0,
                            },
                            RelicAffix {
                            affix_id: 12,
                            cnt: 3,
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
                        affix_id: 3,
                        cnt: 2,
                        step: 3,
                        },
                        RelicAffix {
                        affix_id: 10,
                        cnt: 2,
                        step: 1,
                        },
                        RelicAffix {
                        affix_id: 11,
                        cnt: 2,
                        step: 1,
                        },
                        RelicAffix {
                        affix_id: 12,
                        cnt: 2,
                        step: 0,
                        },
                        ],
                    ..Default::default()      
                },
                BattleRelic {
                    id: 63075,
                    level: 15,
                    main_affix_id: 3,
                    sub_affix_list: vec![
                    RelicAffix {
                    affix_id: 1,
                    cnt: 3,
                    step: 4,
                    },
                    RelicAffix {
                    affix_id: 3,
                    cnt: 1,
                    step: 1,
                    },
                    RelicAffix {
                    affix_id: 4,
                    cnt: 1,
                    step: 2,
                    },
                    RelicAffix {
                    affix_id: 7,
                    cnt: 3,
                    step: 3,
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
                affix_id: 5,
                cnt: 4,
                step: 5,
                },
                RelicAffix {
                affix_id: 6,
                cnt: 1,
                step: 1,
                },
                RelicAffix {
                affix_id: 10,
                cnt: 1,
                step: 2,
                },
                RelicAffix {
                affix_id: 12,
                cnt: 2,
                step: 3,
                },
                ],
            ..Default::default()      
        },
            
                        ], 


                        //Ruan mei
                        1303 => vec![
                                BattleRelic {
                                    id: 61111,
                                    level: 15,
                                    main_affix_id: 1,
                                    sub_affix_list: vec![
                                    RelicAffix {
                                    affix_id: 5,
                                    cnt: 1,
                                    step: 0,
                                    },
                                    RelicAffix {
                                    affix_id: 7,
                                    cnt: 3,
                                    step: 4,
                                    },
                                    RelicAffix {
                                    affix_id: 9,
                                    cnt: 1,
                                    step: 1,
                                    },
                                    RelicAffix {
                                    affix_id: 12,
                                    cnt: 4,
                                    step: 2,
                                    },
                                    ],
                                ..Default::default()      
                            },
                            BattleRelic {
                                id: 61112,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                RelicAffix {
                                affix_id: 4,
                                cnt: 3,
                                step: 0,
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
                                RelicAffix {
                                affix_id: 12,
                                cnt: 3,
                                step: 0,
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
                            affix_id: 6,
                            cnt: 2,
                            step: 3,
                            },
                            RelicAffix {
                            affix_id: 7,
                            cnt: 3,
                            step: 2,
                            },
                            RelicAffix {
                            affix_id: 9,
                            cnt: 3,
                            step: 3,
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
                        affix_id: 2,
                        cnt: 3,
                        step: 3,
                        },
                        RelicAffix {
                        affix_id: 4,
                        cnt: 1,
                        step: 0,
                        },
                        RelicAffix {
                        affix_id: 5,
                        cnt: 1,
                        step: 0,
                        },
                        RelicAffix {
                        affix_id: 12,
                        cnt: 3,
                        step: 1,
                        },
                        ],
                    ..Default::default()      
                },
                BattleRelic {
                    id: 63125,
                    level: 15,
                    main_affix_id: 1,
                    sub_affix_list: vec![
                    RelicAffix {
                    affix_id: 3,
                    cnt: 1,
                    step: 0,
                    },
                    RelicAffix {
                    affix_id: 6,
                    cnt: 4,
                    step: 4,
                    },
                    RelicAffix {
                    affix_id: 7,
                    cnt: 0,
                    step: 1,
                    },
                    RelicAffix {
                    affix_id: 9,
                    cnt: 2,
                    step: 0,
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
                cnt: 3,
                step: 4,
                },
                RelicAffix {
                affix_id: 5,
                cnt: 3,
                step: 4,
                },
                RelicAffix {
                affix_id: 7,
                cnt: 2,
                step: 2,
                },
                RelicAffix {
                affix_id: 11,
                cnt: 1,
                step: 2,
                },
                ],
            ..Default::default()      
        },
            
                        ], 


                  
                        //Harmony mc build 
                        8006 => vec![
                                BattleRelic {
                                    id: 61181,
                                    level: 15,
                                    main_affix_id: 1,
                                    sub_affix_list: vec![
                                    RelicAffix {
                                    affix_id: 4,
                                    cnt: 2,
                                    step: 2,
                                    },
                                    RelicAffix {
                                    affix_id: 5,
                                    cnt: 1,
                                    step: 1,
                                    },
                                    RelicAffix {
                                    affix_id: 7,
                                    cnt: 3,
                                    step: 2,
                                    },
                                    RelicAffix {
                                    affix_id: 12,
                                    cnt: 2,
                                    step: 2,
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
                                affix_id: 4,
                                cnt: 1,
                                step: 0,
                                },
                                RelicAffix {
                                affix_id: 8,
                                cnt: 3,
                                step: 0,
                                },
                                RelicAffix {
                                affix_id: 9,
                                cnt: 1,
                                step: 0,
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
                            id: 61183,
                            level: 15,
                            main_affix_id: 1,
                            sub_affix_list: vec![
                            RelicAffix {
                            affix_id: 1,
                            cnt: 2,
                            step: 2,
                            },
                            RelicAffix {
                            affix_id: 7,
                            cnt: 4,
                            step: 3,
                            },
                            RelicAffix {
                            affix_id: 8,
                            cnt: 1,
                            step: 0,
                            },
                            RelicAffix {
                            affix_id: 9,
                            cnt: 1,
                            step: 0,
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
                        cnt: 3,
                        step: 2,
                        },
                        RelicAffix {
                        affix_id: 2,
                        cnt: 2,
                        step: 1,
                        },
                        RelicAffix {
                        affix_id: 15,
                        cnt: 2,
                        step: 4,
                        },
                        RelicAffix {
                        affix_id: 12,
                        cnt: 2,
                        step: 3,
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
                    affix_id: 5,
                    cnt: 2,
                    step: 1,
                    },
                    RelicAffix {
                    affix_id: 6,
                    cnt: 2,
                    step: 1,
                    },
                    RelicAffix {
                    affix_id: 10,
                    cnt: 2,
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
                id: 63076,
                level: 15,
                main_affix_id: 1,
                sub_affix_list: vec![
                RelicAffix {
                affix_id: 1,
                cnt: 2,
                step: 2,
                },
                RelicAffix {
                affix_id: 2,
                cnt: 0,
                step: 2,
                },
                RelicAffix {
                affix_id: 10,
                cnt: 1,
                step: 0,
                },
                RelicAffix {
                affix_id: 11,
                cnt: 3,
                step: 3,
                },
                ],
            ..Default::default()      
        },
            
                        ], 


            
                        _ => vec![]
                    },
                    equipment_list: match avatar.id {
                        1310 => vec![ // FireFly Aeon
                            BattleEquipment {
                                id: 23025,
                                level: 80,
                                promotion: 6,
                                rank: 5
                                }
                        ],
                        1301 => vec![ // Gallagher Aeon
                            BattleEquipment {
                                id: 21035,
                                level: 80,
                                promotion: 6,
                                rank: 5
                                }
                        ],
                        8006 => vec![ // HMC Aeon
                            BattleEquipment {
                                id: 21004,
                                level: 80,
                                promotion: 6,
                                rank: 5
                                }
                        ],
                        1303 => vec![ // Ruan Mei Aeon
                            BattleEquipment {
                                id: 21004,
                                level: 80,
                                promotion: 6,
                                rank: 5
                                }
                        ],
                        _ => vec![]
                    },  
                    ..Default::default()
                })
                .collect(),
            monster_wave_list: globals
                .monster_wave_list
                .iter()
                .map(|monster_list| SceneMonsterWave {
                    acpannfhach: Some(
                        Kjfnknacfin {
                            level: 95,
                            ..Default::default()
                        }
                    ),
                    monster_list: monster_list
                        .iter()
                        .map(|id| SceneMonsterData {
                            monster_id: *id,
                            //max_hp: 650000,
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
