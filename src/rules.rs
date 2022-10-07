use crate::{evm::*, types::*};

pub fn check_rules(peephole: &Bytecode, pushdata: Vec<Option<PushData>>) -> Bytecode {
    let new_bytecode: Bytecode = match peephole[..] {
        // Redundant swaps on commutative operations
        [ByteData {
            opcode: Some(Opcode::Swap1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Add),
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Swap1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Mul),
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Swap1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Eq),
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Swap1),
            ..
        }, ByteData {
            opcode: Some(Opcode::And),
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Swap1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Or),
            ..
        }] => peephole[1..].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Swap1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Xor),
            ..
        }] => peephole[1..].to_vec(),

        // Operations involving an expression and itself
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::And),
            ..
        }] => [].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Or),
            ..
        }] => [].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Xor),
            ..
        }] => [
            ByteData {
                pc: peephole[0].pc,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: peephole[1].pc,
                opcode: None,
                pushdata: Some(String::from("00")),
                kind: ByteKind::PushData
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Sub),
            ..
        }] => [
            ByteData {
                pc: peephole[0].pc,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: peephole[1].pc,
                opcode: None,
                pushdata: Some(String::from("00")),
                kind: ByteKind::PushData
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Eq),
            ..
        }] => [
            ByteData {
                pc: peephole[0].pc,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: peephole[1].pc,
                opcode: None,
                pushdata: Some(String::from("01")),
                kind: ByteKind::PushData
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Lt),
            ..
        }] => [
            ByteData {
                pc: peephole[0].pc,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: peephole[1].pc,
                opcode: None,
                pushdata: Some(String::from("00")),
                kind: ByteKind::PushData
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Slt),
            ..
        }] => [
            ByteData {
                pc: peephole[0].pc,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: peephole[1].pc,
                opcode: None,
                pushdata: Some(String::from("00")),
                kind: ByteKind::PushData
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Gt),
            ..
        }] => [
            ByteData {
                pc: peephole[0].pc,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: peephole[1].pc,
                opcode: None,
                pushdata: Some(String::from("00")),
                kind: ByteKind::PushData
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Sgt),
            ..
        }] => [
            ByteData {
                pc: peephole[0].pc,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: peephole[1].pc,
                opcode: None,
                pushdata: Some(String::from("00")),
                kind: ByteKind::PushData
            }
        ].to_vec(),
        [ByteData {
            opcode: Some(Opcode::Dup1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Mod),
            ..
        }] => [
            ByteData {
                pc: peephole[0].pc,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: peephole[1].pc,
                opcode: None,
                pushdata: Some(String::from("00")),
                kind: ByteKind::PushData
            }
        ].to_vec(),

        // Duplicate pushes
        [ByteData {
            opcode: Some(Opcode::Push1),
            ..
        }, ByteData {
            opcode: Some(Opcode::Push1),
            ..
        }] if pushdata[0].as_ref().unwrap() == pushdata[1].as_ref().unwrap() => [
            ByteData {
                pc: peephole[0].pc,
                opcode: Some(Opcode::Push1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
            ByteData {
                pc: peephole[1].pc,
                opcode: None,
                pushdata: Some(pushdata[0].as_ref().unwrap().to_string()),
                kind: ByteKind::PushData
            },
            ByteData {
                pc: peephole[1].pc + 1,
                opcode: Some(Opcode::Dup1),
                pushdata: None,
                kind: ByteKind::Opcode
            },
        ].to_vec(),
        _ => peephole[..].to_vec(),
    };
    new_bytecode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commutative_swaps() {
        // Swap1, Add => Add
        let peephole: Bytecode = vec![
            ByteData {
                pc: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 5,
                opcode: Some(Opcode::Add),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            pc: 5,
            opcode: Some(Opcode::Add),
            pushdata: None,
            kind: ByteKind::Opcode,
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Swap1, Mul => Mul
        let peephole: Bytecode = vec![
            ByteData {
                pc: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 5,
                opcode: Some(Opcode::Mul),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            pc: 5,
            opcode: Some(Opcode::Mul),
            pushdata: None,
            kind: ByteKind::Opcode,
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Swap1, Eq => Eq
        let peephole: Bytecode = vec![
            ByteData {
                pc: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 5,
                opcode: Some(Opcode::Eq),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            pc: 5,
            opcode: Some(Opcode::Eq),
            pushdata: None,
            kind: ByteKind::Opcode,
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Swap1, And => And
        let peephole: Bytecode = vec![
            ByteData {
                pc: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 5,
                opcode: Some(Opcode::And),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            pc: 5,
            opcode: Some(Opcode::And),
            pushdata: None,
            kind: ByteKind::Opcode,
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Swap1, Or => Or
        let peephole: Bytecode = vec![
            ByteData {
                pc: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 5,
                opcode: Some(Opcode::Or),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            pc: 5,
            opcode: Some(Opcode::Or),
            pushdata: None,
            kind: ByteKind::Opcode,
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Swap1, Xor => Xor
        let peephole: Bytecode = vec![
            ByteData {
                pc: 4,
                opcode: Some(Opcode::Swap1),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
            ByteData {
                pc: 5,
                opcode: Some(Opcode::Xor),
                pushdata: None,
                kind: ByteKind::Opcode,
            },
        ];
        let optimized_peephole: Bytecode = vec![ByteData {
            pc: 5,
            opcode: Some(Opcode::Xor),
            pushdata: None,
            kind: ByteKind::Opcode,
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));
    }

    #[test]
    fn test_dup_expression_operations() {
        // Dup1, And => []
        let peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: Some(Opcode::And),
            pushdata: None,
            kind: ByteKind::Opcode
        }];
        let optimized_peephole: Vec<ByteData> = Vec::new();
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Dup1, Or => []
        let peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: Some(Opcode::Or),
            pushdata: None,
            kind: ByteKind::Opcode
        }];
        let optimized_peephole: Vec<ByteData> = Vec::new();
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Dup1, Xor => Push1, 00
        let peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: Some(Opcode::Xor),
            pushdata: None,
            kind: ByteKind::Opcode
        }];
        let optimized_peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Push1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: None,
            pushdata: Some(String::from("00")),
            kind: ByteKind::PushData
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Dup1, Sub => Push1, 00
        let peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: Some(Opcode::Sub),
            pushdata: None,
            kind: ByteKind::Opcode
        }];
        let optimized_peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Push1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: None,
            pushdata: Some(String::from("00")),
            kind: ByteKind::PushData
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Dup1, Eq => Push1, 01
        let peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: Some(Opcode::Eq),
            pushdata: None,
            kind: ByteKind::Opcode
        }];
        let optimized_peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Push1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: None,
            pushdata: Some(String::from("01")),
            kind: ByteKind::PushData
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Dup1, Lt => Push1, 00
        let peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: Some(Opcode::Lt),
            pushdata: None,
            kind: ByteKind::Opcode
        }];
        let optimized_peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Push1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: None,
            pushdata: Some(String::from("00")),
            kind: ByteKind::PushData
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Dup1, Slt => Push1, 00
        let peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: Some(Opcode::Slt),
            pushdata: None,
            kind: ByteKind::Opcode
        }];
        let optimized_peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Push1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: None,
            pushdata: Some(String::from("00")),
            kind: ByteKind::PushData
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Dup1, Gt => Push1, 00
        let peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: Some(Opcode::Gt),
            pushdata: None,
            kind: ByteKind::Opcode
        }];
        let optimized_peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Push1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: None,
            pushdata: Some(String::from("00")),
            kind: ByteKind::PushData
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Dup1, Sgt => Push1, 00
        let peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: Some(Opcode::Sgt),
            pushdata: None,
            kind: ByteKind::Opcode
        }];
        let optimized_peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Push1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: None,
            pushdata: Some(String::from("00")),
            kind: ByteKind::PushData
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));

        // Dup1, Mod => Push1, 00
        let peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: Some(Opcode::Mod),
            pushdata: None,
            kind: ByteKind::Opcode
        }];
        let optimized_peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Push1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: None,
            pushdata: Some(String::from("00")),
            kind: ByteKind::PushData
        }];
        let pushdata: Vec<Option<PushData>> = Vec::new();
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));
    }

    #[test]
    fn test_duplicate_pushes() {
        // Push1, X, Push1, X => Push1, X, Dup1
        let peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Push1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: Some(Opcode::Push1),
            pushdata: None,
            kind: ByteKind::Opcode
        }];
        let optimized_peephole = vec![ByteData {
            pc: 4,
            opcode: Some(Opcode::Push1),
            pushdata: None,
            kind: ByteKind::Opcode
        }, ByteData {
            pc: 5,
            opcode: None,
            pushdata: Some(String::from("00")),
            kind: ByteKind::PushData
        }, ByteData {
            pc: 6,
            opcode: Some(Opcode::Dup1),
            pushdata: None,
            kind: ByteKind::Opcode
        }];
        let pushdata: Vec<Option<PushData>> = vec![Some(String::from("00")), Some(String::from("00"))];
        assert_eq!(optimized_peephole, check_rules(&peephole, pushdata));
    }
}
