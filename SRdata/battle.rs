use super::*;
use crate::game::globals;
use std::collections::HashMap;

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
            stage_id: 30305042,//虚构叙事 向壁虚造
            logic_random_seed: 4444,
            ohfkoaahoib: 4,
            battle_id: 1,

            buff_list:vec![
            BattleBuff {
                id: 100301,// 姬子秘技 生效！
                owner_index: 0,//lineupindex(0,1,2,3 from left to right),
                level: 1,
                wave_flag: 1<<2,//X(x= number of enemies on wave),
                dynamic_values: HashMap::from([(String::from("SkillIndex"), 0 as f32)]),
                ..Default::default()
                },
                BattleBuff {
                id: 131401,// 翡翠秘技 生效！
                owner_index: 1,//lineupindex(0,1,2,3 from left to right),
                level: 1,
                wave_flag: 1<<2,//X(x= number of enemies on wave),
                dynamic_values: HashMap::from([(String::from("SkillIndex"), 0 as f32)]),
                ..Default::default()
                },
                BattleBuff {
                id: 130301,// 阮梅秘技 生效！
                owner_index: 2,//lineupindex(0,1,2,3 from left to right),
                level: 1,
                wave_flag: 1<<2,//X(x= number of enemies on wave),
                dynamic_values: HashMap::from([(String::from("SkillIndex"), 0 as f32)]),
                ..Default::default()
                },
                BattleBuff {
                id: 130401,// 砂金秘技 生效！
                owner_index: 3,//lineupindex(0,1,2,3 from left to right),
                level: 1,
                wave_flag: 1<<2,//X(x= number of enemies on wave),
                dynamic_values: HashMap::from([(String::from("SkillIndex"), 0 as f32)]),
                ..Default::default()
                }
            ],

           battle_avatar_list: player_info
                .lineup
                .avatar_list
                .iter()
                .map(|avatar| BattleAvatar {
                    index: avatar.slot,
                    id: avatar.id,
                    level: 80,
                    promotion: 6,
                    rank: match avatar.id {//星魂
                        1309 => 1,//知更鸟Robin
                        1202 => 6,//停云Tingyun
                        1106 => 6,//佩拉pela
                        1306 => 0,//花火Sparkle
                        1308 => 0,//黄泉Acheron
                        1208 => 2,//符玄Fuxuan
                        1112 => 0,//托帕Topaz & Numby
                        1305 => 0,//真理医生Dr.Ratio
                        1304 => 0,//砂金Aventurine
                        1315 => 0,//波提欧Boothill
                        1303 => 0,//阮梅RuanMei
                        1210 => 6,//桂乃芬GuiNaiFen
                        1203 => 0,//罗刹Luocha
                        1310 => 2,//流萤Firefly
                        8006 => 6,//同谐主角The Harmony Trailblazer
                        1314 => 0,//翡翠Jade
                        1003 => 1, //姬子Himiko
                        _ => 0
                    },
                    hp: 10000,//血量100%
                    avatar_type: 3,
                    sp: Some(AmountInfo {
                        cur_amount: 5000,//当前能量50%
                        max_amount: 10000,//最大能量100%
                    }),

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

                    relic_list: match avatar.id {
                        1003 => vec![//姬子 4大公 2萨尔索图
                            BattleRelic {
                              id: 61151,
                              level: 15,
                              main_affix_id: 1,
                              sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 6,    
                                    cnt: 3,
                                    step: 3,
                                },
                                RelicAffix {
                                    affix_id: 8,
                                    cnt: 3,
                                    step: 6,
                                },
                                RelicAffix {
                                    affix_id: 9,
                                    cnt: 1,
                                    step: 0,
                                },
                                RelicAffix {
                                    affix_id: 10,
                                    cnt: 1,
                                    step: 0,
                                },
                              ],
                              ..Default::default()
                           },
                            BattleRelic {
                              id: 61152,
                              level: 15,
                              main_affix_id: 1,
                              sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 5,
                                    cnt: 1,
                                    step: 0,
                                },
                                RelicAffix {
                                    affix_id: 8,
                                    cnt: 3,
                                    step: 4,
                                },
                                RelicAffix {
                                    affix_id: 11,
                                    cnt: 2,
                                    step: 2,
                                },
                                RelicAffix {
                                    affix_id: 9,
                                    cnt: 2,
                                    step: 3,
                                },
                              ],
                              ..Default::default()
                            },
                            BattleRelic {
                                id: 61154,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 3,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 3,
                                        step: 1,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 2,
                                        step: 1,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 2,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },
                            BattleRelic {
                                id: 63065,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 1,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 3,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 2,
                                        step: 0,
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
                                id: 63066,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 4,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 0,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 1,
                                        step: 1,
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
                                id: 61153,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 3,
                                        step: 1,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id:9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },  
                        ],


                        1314 => vec![//翡翠 4量子 2萨尔索图
                            BattleRelic {
                              id: 63065,
                              level: 15,
                              main_affix_id: 9,
                              sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 4,    
                                    cnt: 1,
                                    step: 2,
                                },
                                RelicAffix {
                                    affix_id: 5,
                                    cnt: 2,
                                    step: 2,
                                },
                                RelicAffix {
                                    affix_id: 6,
                                    cnt: 2,
                                    step: 2,
                                },
                                RelicAffix {
                                    affix_id: 8,
                                    cnt: 3,
                                    step: 3,
                                },
                              ],
                              ..Default::default()
                           },
                            BattleRelic {
                              id: 63066,
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
                                    cnt: 2,
                                    step: 2,
                                },
                                RelicAffix {
                                    affix_id: 8,
                                    cnt: 3,
                                    step: 3,
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
                                id: 61083,
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
                                        cnt: 2,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 4,
                                        step: 8,
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
                                id: 61084,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 3,
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
                                id: 61082,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 3,
                                        step: 3,
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
                                id: 61081,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 3,
                                        step: 3,
                                    },
                                    RelicAffix {
                                        affix_id:9,
                                        cnt: 2,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },  
                        ],


                        1112 => vec![//托帕 4追击 2萨尔索图
                            BattleRelic {
                              id: 61151,
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
                                    cnt: 4,
                                    step: 8,
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
                              id: 61152,
                              level: 15,
                              main_affix_id: 1,
                              sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 5,
                                    cnt: 2,
                                    step: 4,
                                },
                                RelicAffix {
                                    affix_id: 8,
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
                                    cnt: 2,
                                    step: 4,
                                },
                              ],
                              ..Default::default()
                            },
                            BattleRelic {
                                id: 61153,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
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
                                id: 61154,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
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
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },
                            BattleRelic {
                                id: 63066,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 3,
                                        step: 6,
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
                                id: 63065,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id:11,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },  
                        ],
                        1305 => vec![//真理 4死水 2萨尔索图
                            BattleRelic {
                              id: 61171,
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
                                    cnt: 3,
                                    step: 6,
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
                              ],
                              ..Default::default()
                           },
                            BattleRelic {
                              id: 61172,
                              level: 15,
                              main_affix_id: 1,
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
                                id: 61173,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 1,
                                        step: 2,
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
                                id: 61174,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },
                            BattleRelic {
                                id: 63066,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
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
                                id: 63065,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },  
                        ],
                        1303 => vec![//阮梅 2钟表匠2信使2龙骨
                            BattleRelic {
                              id: 61181,
                              level: 15,
                              main_affix_id: 1,
                              sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 2,
                                    cnt: 2,
                                    step: 3,
                                },
                                RelicAffix {
                                    affix_id: 7,
                                    cnt: 4,
                                    step: 2,
                                },
                                RelicAffix {
                                    affix_id: 8,
                                    cnt: 1,
                                    step: 1,
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
                              id: 61142,
                              level: 15,
                              main_affix_id: 1,
                              sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 1,
                                    cnt: 1,
                                    step: 1,
                                },
                                RelicAffix {
                                    affix_id: 7,
                                    cnt: 1,
                                    step: 2,
                                },
                                RelicAffix {
                                    affix_id: 10,
                                    cnt: 1,
                                    step: 1,
                                },
                                RelicAffix {
                                    affix_id: 12,
                                    cnt: 5,
                                    step: 8,
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
                                        affix_id: 2,
                                        cnt: 2,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 2,
                                        step: 1,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 5,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 1,
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
                                        cnt: 2,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 0,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 4,
                                        step: 8,
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
                                id: 63105,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 3,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 3,
                                        step: 5,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
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
                            BattleRelic {
                                id: 63106,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 1,
                                        step: 1,
                                    },
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 2,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 4,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },  
                        ],
                        1208 => vec![//符玄 2铁卫2时者 2龙骨
                            BattleRelic {
                              id: 63106,
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
                                    cnt: 6,
                                    step: 12,
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
                                id: 63105,
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
                                        cnt: 6,
                                        step: 12,
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
                                id: 61061,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
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
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
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
                                id: 61062,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
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
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
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
                                id: 61133,
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
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 5,
                                        step: 10,
                                    },
                                ],
                                ..Default::default()
                            },
                            BattleRelic {
                                id: 61134,
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
                        ],
                        1304 => vec![//砂金 4死水 2萨尔索图
                           BattleRelic {
                               id: 61171,
                               level: 15,
                               main_affix_id: 1,
                               sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 4,
                                    cnt: 1,
                                    step: 0,
                                },
                                RelicAffix {
                                    affix_id: 7,
                                    cnt: 2,
                                    step: 4,
                                },
                                RelicAffix {
                                    affix_id: 9,
                                    cnt: 4,
                                    step: 5,
                                },
                                RelicAffix {
                                    affix_id: 10,
                                    cnt: 2,
                                    step: 2,
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
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 2,
                                        step: 0,
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
                                id: 63065,
                                level: 15,
                                main_affix_id: 3,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 3,
                                        step: 3,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 2,
                                        step: 4,
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
                                id: 63066,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 0,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 1,
                                        step: 1,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 4,
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
                                id: 61174,
                                level: 15,
                                main_affix_id: 3,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 2,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 3,
                                        step: 3,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
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
                            BattleRelic {
                                id: 61173,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 1,
                                        step: 1,
                                    },
                                    RelicAffix {
                                        affix_id: 12,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },
                        ],
                        1006 => vec![//银狼 4风套 2龙骨
                        BattleRelic {
                           id: 61101,
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
                                cnt: 6,
                                step: 12,
                            },
                            RelicAffix {
                                affix_id: 10,
                                cnt: 1,
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
                        BattleRelic {
                            id: 61102,
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
                                    cnt: 6,
                                    step: 12,
                                },
                                RelicAffix {
                                    affix_id: 10,
                                    cnt: 1,
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
                        BattleRelic {
                            id: 61103,
                            level: 15,
                            main_affix_id: 7,
                            sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 6,
                                    cnt: 1,
                                    step: 2,
                                },
                                RelicAffix {
                                    affix_id: 7,
                                    cnt: 6,
                                    step: 12,
                                },
                                RelicAffix {
                                    affix_id: 11,
                                    cnt: 1,
                                    step: 2,
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
                            id: 61104,
                            level: 15,
                            main_affix_id: 4,
                            sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 4,
                                    cnt: 4,
                                    step: 8,
                                },
                                RelicAffix {
                                    affix_id: 6,
                                    cnt: 3,
                                    step: 6,
                                },
                                RelicAffix {
                                    affix_id: 10,
                                    cnt: 1,
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
                        BattleRelic {
                            id: 63106,
                            level: 15,
                            main_affix_id: 2,
                            sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 6,
                                    cnt: 2,
                                    step: 4,
                                },
                                RelicAffix {
                                    affix_id: 7,
                                    cnt: 5,
                                    step: 10,
                                },
                                RelicAffix {
                                    affix_id: 10,
                                    cnt: 1,
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
                        BattleRelic {
                            id: 63105,
                            level: 15,
                            main_affix_id: 1,
                            sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 6,
                                    cnt: 2,
                                    step: 4,
                                },
                                RelicAffix {
                                    affix_id: 7,
                                    cnt: 5,
                                    step: 10,
                                },
                                RelicAffix {
                                    affix_id: 10,
                                    cnt: 1,
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
                        1306 => vec![//花火 4风套 2匹诺康尼
                            BattleRelic {
                               id: 61101,
                               level: 15,
                               main_affix_id: 1,
                               sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 6,
                                    cnt: 1,
                                    step: 2,
                                },
                                RelicAffix {
                                    affix_id: 9,
                                    cnt: 3,
                                    step: 10,
                                },
                                RelicAffix {
                                    affix_id: 10,
                                    cnt: 1,
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
                            BattleRelic {
                                id: 61102,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 5,
                                        step: 10,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 1,
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
                            BattleRelic {
                                id: 61103,
                                level: 15,
                                main_affix_id: 5,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 5,
                                        step: 10,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 1,
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
                            BattleRelic {
                                id: 61104,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 1,
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
                            BattleRelic {
                                id: 63126,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 5,
                                        step: 10,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 1,
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
                            BattleRelic {
                                id: 63125,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 1,
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
                        1203 => vec![//罗刹 4过客 2龙骨
                            BattleRelic {
                               id: 61011,
                               level: 15,
                               main_affix_id: 1,
                               sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 6,
                                    cnt: 3,
                                    step: 6,
                                },
                                RelicAffix {
                                    affix_id: 7,
                                    cnt: 3,
                                    step: 6,
                                },
                                RelicAffix {
                                    affix_id: 5,
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
                                id: 61012,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                   ..Default::default()
                            },
                            BattleRelic {
                                id: 61013,
                                level: 15,
                                main_affix_id: 6,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 4,
                                        step: 8,
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
                                ],
                                ..Default::default()
                            },
                            BattleRelic {
                                id: 61014,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 1,
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
                            BattleRelic {
                                id: 63106,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
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
                                id: 63105,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },
                        ],
                        1210 => vec![//桂乃芬 4风套 2翁瓦克
                            BattleRelic {
                               id: 61101,
                               level: 15,
                               main_affix_id: 1,
                               sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 6,
                                    cnt: 3,
                                    step: 6,
                                },
                                RelicAffix {
                                    affix_id: 7,
                                    cnt: 3,
                                    step: 6,
                                },
                                RelicAffix {
                                    affix_id: 10,
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
                                id: 61102,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                   ..Default::default()
                            },
                            BattleRelic {
                                id: 61103,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 4,
                                        step: 8,
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
                                ],
                                ..Default::default()
                            },
                            BattleRelic {
                                id: 61104,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 1,
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
                            BattleRelic {
                                id: 63086,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
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
                                id: 63085,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },
                        ],
                        1106 => vec![//佩拉 4风套 2翁瓦克
                            BattleRelic {
                               id: 61101,
                               level: 15,
                               main_affix_id: 1,
                               sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 6,
                                    cnt: 3,
                                    step: 6,
                                },
                                RelicAffix {
                                    affix_id: 7,
                                    cnt: 3,
                                    step: 6,
                                },
                                RelicAffix {
                                    affix_id: 10,
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
                                id: 61102,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                   ..Default::default()
                            },
                            BattleRelic {
                                id: 61103,
                                level: 15,
                                main_affix_id: 7,
                                sub_affix_list: vec![
                            
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
                                        affix_id: 4,
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
                                id: 61104,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 1,
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
                            BattleRelic {
                                id: 63086,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
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
                                id: 63085,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                            
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            },
                        ],
                        1308 => vec![//黄泉 4死水 2出云
                            BattleRelic {
                              id: 61171,
                              level: 15,
                              main_affix_id: 1,
                              sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 2,
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
                                    cnt: 3,
                                    step: 6,
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
                                    affix_id: 2,
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
                                    cnt: 3,
                                    step: 6,
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
                                    affix_id: 2,
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
                                    cnt: 6,
                                    step: 12,
                                },
                                RelicAffix {
                                    affix_id: 10,
                                    cnt: 1,
                                    step: 2,
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
                                    affix_id: 7,
                                    cnt: 1,
                                    step: 2,
                                },
                                RelicAffix {
                                    affix_id: 8,
                                    cnt: 3,
                                    step: 6,
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
                              id: 63146,
                              level: 15,
                              main_affix_id: 4,
                              sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 2,
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
                              id: 63145,
                              level: 15,
                              main_affix_id: 7,
                              sub_affix_list: vec![
                        
                                RelicAffix {
                                    affix_id: 2,
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
                        ],
                        1310 => vec![ // 流萤 4铁骑 2炼宫
                            BattleRelic {
                                id: 61191,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
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
                                id: 61192,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
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
                                id: 61193,
                                level: 15,
                                main_affix_id: 2,//攻击衣
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
                                        cnt: 3,
                                        step: 6,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61194,
                                level: 15,
                                main_affix_id: 2,//攻击鞋
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 1,
                                        step: 1,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 1,
                                        step: 2,
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
                                main_affix_id: 2,//攻击球
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
                                        affix_id: 9,
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
                                id: 63166,
                                level: 15,
                                main_affix_id: 1,//击破特攻
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 3,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],
                            8006 => vec![ // 同谐主角 4钟表 2盗贼
                            BattleRelic {
                                id: 61181,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 5,
                                        step: 10,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
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
                                id: 61182,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
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
                                id: 61183,
                                level: 15,
                                main_affix_id: 5,//爆伤衣
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
                                id: 61184,
                                level: 15,
                                main_affix_id: 4,//速度鞋
                                sub_affix_list: vec![
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
                                        cnt: 1,
                                        step: 2,
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
                                id: 63075,
                                level: 15,
                                main_affix_id: 4,
                                sub_affix_list: vec![
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
                                id: 63076,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
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
                                        cnt: 5,
                                        step: 10,
                                    },
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 2,
                                        step: 4,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],
                        1309 => vec![ // 知更鸟 2快枪手2幽囚2翁瓦克
                            BattleRelic {
                                id: 61021,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
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
                                id: 61022,
                                level: 15,
                                main_affix_id: 1,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 1,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 5,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 2,
                                        step: 4,
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
                                        affix_id: 3,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 10,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            },

                            BattleRelic {
                                id: 61164,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
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
                                id: 63085,
                                level: 15,
                                main_affix_id: 2,
                                sub_affix_list: vec![
                                    RelicAffix {
                                        affix_id: 7,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 6,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 11,
                                        cnt: 2,
                                        step: 4,
                                    },
                                    RelicAffix {
                                        affix_id: 4,
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
                                        affix_id: 5,
                                        cnt: 4,
                                        step: 8,
                                    },
                                    RelicAffix {
                                        affix_id: 2,
                                        cnt: 3,
                                        step: 6,
                                    },
                                    RelicAffix {
                                        affix_id: 8,
                                        cnt: 1,
                                        step: 2,
                                    },
                                    RelicAffix {
                                        affix_id: 9,
                                        cnt: 1,
                                        step: 2,
                                    },
                                ],
                                ..Default::default()
                            }
                        ],
                        /*1303 => vec![ // 4 PC Watchmaker, Master of Dream Machinations & 2pc- Sprightly Vonwacq
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
                        ],*/
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
                        _ => vec![] // default - nothing
                    },
                    equipment_list: match avatar.id {
                        1112 => vec![
                            BattleEquipment {
                                id: 23016,//托帕专武Topaz's lc
                                level: 80,
                                promotion: 6,
                                rank: 1//叠影
                            }
                        ],
                        1305 => vec![
                            BattleEquipment {
                                id: 23020,//纯粹思维的洗礼Dr.Ratio's lc
                                level: 80,
                                promotion: 6,
                                rank: 1//叠影
                            }
                        ],
                        1208 => vec![
                            BattleEquipment {
                                id: 23011,//命她已闭上双眼Fuxuan's lc
                                level: 80,
                                promotion: 6,
                                rank: 1//叠影
                            }
                        ],
                        1304 => vec![ //砂金 光锥配置
                            BattleEquipment {
                                id: 21039,//织造命运之线
                                level: 80,
                                promotion: 6,
                                rank: 5//叠影
                            }
                        ],
                        1006 => vec![
                            BattleEquipment {
                                id: 22000,//新手任务开始前SilverWolf's lc
                                level: 80,
                                promotion: 6,
                                rank: 5 //叠影
                            }
                        ],
                        1306 => vec![
                            BattleEquipment {
                                id: 23021,//花火专武Sparkle's lc
                                level: 80,
                                promotion: 6,
                                rank: 1 //叠影
                            }
                        ],
                        1106 => vec![
                            BattleEquipment {
                                id: 21015,//决心如汗珠pela's lc
                                level: 80,
                                promotion: 6,
                                rank: 5 //叠影
                            }
                        ],
                        1210 => vec![
                            BattleEquipment {
                                id: 22000,//新手任务GuiNaiFen's lc
                                level: 80,
                                promotion: 6,
                                rank: 5 //叠影
                            }
                        ],
                        1308 => vec![
                            BattleEquipment {
                                id: 23024,//行于流逝的岸Acheron's lc
                                level: 80,
                                promotion: 6,
                                rank: 1 //叠影
                            }
                        ],
                        1315 => vec![ // LC- Cruising in the Stellar Sea- 24001- S1
                            BattleEquipment {
                                id: 23027,//波提欧专武Boothill's lc
                                level: 80,
                                promotion: 6,
                                rank: 1
                            }
                        ],
                        1309 => vec![ // LC- For Tomorrow's Journey- S1- 22002
                            BattleEquipment {
                                id: 23026,//知更鸟专武Robin's lc
                                level: 80,
                                promotion: 6,
                                rank: 1
                            }
                        ],
                        1203 => vec![ // 时节不居
                            BattleEquipment {
                                id: 23013,//时节不居 lc
                                level: 80,
                                promotion: 6,
                                rank: 1
                            }
                        ],
                        1310 => vec![ // 流萤专武
                            BattleEquipment {
                                id: 23025,//流萤专武 lc
                                level: 80,
                                promotion: 6,
                                rank: 1
                            }
                        ],
                        1003 => vec![ // 姬子
                            BattleEquipment {
                                id: 21040,//银河沦陷日 lc
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],
                        1314 => vec![ // 翡翠
                            BattleEquipment {
                                id: 23028,//翡翠专武 lc
                                level: 80,
                                promotion: 6,
                                rank: 1
                            }
                        ],
                        8006 => vec![ // 同谐主
                            BattleEquipment {
                                id: 21004,//记忆中的模样 lc
                                level: 80,
                                promotion: 6,
                                rank: 1
                            }
                        ],
                        1303 => vec![ // 阮梅LC- Memories of the Past- S1- 21004
                            BattleEquipment {
                                id: 21018,//DDD lc
                                level: 80,
                                promotion: 6,
                                rank: 5
                            }
                        ],
                        1217 => vec![ // LC- Hey, Over Here- S1- 22001
                            BattleEquipment {
                                id: 22001,
                                level: 80,
                                promotion: 6,
                                rank: 1
                            }
                        ],
                        _ => vec![] // default - nothing
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
                            level: 85, // monsters level
                            ..Default::default()
                        }
                    ),                    
                    monster_list: monster_list
                        .iter()
                        .map(|id| SceneMonsterData {
                            monster_id: *id,
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
