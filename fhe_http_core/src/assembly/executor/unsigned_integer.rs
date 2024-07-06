use crate::configs::errors::AsmError;
use crate::configs::instructions::{Instruction, InstructionLine};
use crate::fhe_traits::computable::{Computable, Shiftable};
use crate::fhe_traits::encryptable::Encryptable;
use std::collections::HashMap;
use tfhe::{ClientKey, FheUint64};

macro_rules! impl_execute_signed_int {
    ($t:ty, $fhe_ty:ty) => {
        paste::item! {
            pub fn [<execute_asm_ $t>](
                asm: &Vec<InstructionLine>,
                args: &HashMap<String, $fhe_ty>,
                client_key: &ClientKey,
            ) -> Result<$fhe_ty, AsmError> {
                let mut register_map: HashMap<&String, $fhe_ty> = HashMap::new();

                for line in asm {
                    let operands = &line.operands;
                    let register = &line.register;
                    match line.instruction {
                        Instruction::VAR => {
                            let arg = args.get(&operands[0]).unwrap();
                            register_map.insert(register, arg.clone());
                        }
                        Instruction::MOV => {
                            let operand = operands[0].parse::<$t>().unwrap();
                            let arg = operand.val_encrypt(client_key);
                            if let Ok(arg) = arg {
                                register_map.insert(register, arg);
                            } else {
                                return Err(AsmError::ExecutionError("Encryption error".to_string()));
                            }
                        }
                        Instruction::NEG => {
                            let arg = register_map.get(&operands[0]).unwrap();
                            let res = arg.neg();
                            register_map.insert(register, res);
                        }

                        Instruction::NOT => {
                            let arg = register_map.get(&operands[0]).unwrap();
                            let res = arg.not();
                            register_map.insert(register, res);
                        }

                        Instruction::ADD => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.add(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::SUB => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.sub(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::MUL => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.mul(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::DIV => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.div(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::REM => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.rem(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::AND => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.and(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::OR => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.or(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::XOR => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.xor(arg2);
                            register_map.insert(register, res);
                        }
                        Instruction::SHR => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.shr(arg2);
                            register_map.insert(register, res);
                        }
                        Instruction::SHL => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.shl(arg2);
                            register_map.insert(register, res);
                        }
                        Instruction::OUT => {
                            if let Some(arg) = register_map.get(register) {
                                return Ok(arg.clone());
                            } else {
                                let msg = format!("Register {} not found", register);
                                return Err(AsmError::OutputError(msg));
                            }
                        }
                    }
                }
                Err(AsmError::OutputError("Output not defined".to_string()))
            }
        }
    };
}

impl_execute_signed_int!(u64, FheUint64);
