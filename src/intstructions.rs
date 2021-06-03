use std::collections::HashMap;

pub struct Instruction {
    pub instr_set: HashMap<u8, String>,
    pub instr_time: [Vec<String>; 3],
    pub instr_length: [Vec<String>; 3],
    pub instr_type: [Vec<String>; 2]
}

impl Instruction {
    pub fn new() -> Instruction {
        let instrucitions: Instruction = Instruction{
            instr_set: Instruction::opcodes(),
            instr_time: Instruction::time_instr(),
            instr_length: Instruction::length_instr(),
            instr_type: Instruction::type_instr()
        };
        instrucitions
    }

    pub fn opcodes() -> HashMap<u8, String> {
        let mut instr_set: HashMap<u8, String> = HashMap::new();
    
        // load reg2 -> reg1
        // reg1 == reg2 => NOP
        let nop: [u8; 7] = [192, 201, 210, 219, 228, 237, 246];
        // reg1 != reg2 => Lrr
        for reg1 in (192..241).step_by(8) {
            for reg2 in 0..7 {
                let op = reg1 + reg2;
                if nop.contains(&op) {
                    instr_set.insert(op, "NOP".to_string());
                } else {
                    if reg1 == 192 {
                        match reg2 {
                            1 => instr_set.insert(op, "LAB".to_string()),
                            2 => instr_set.insert(op, "LAC".to_string()),
                            3 => instr_set.insert(op, "LAD".to_string()),
                            4 => instr_set.insert(op, "LAE".to_string()),
                            5 => instr_set.insert(op, "LAH".to_string()),
                            6 => instr_set.insert(op, "LAL".to_string()),
                            _ => continue
                        };
                    } else if reg1 == 200 {
                        match reg2 {
                            0 => instr_set.insert(op, "LBA".to_string()),
                            2 => instr_set.insert(op, "LBC".to_string()),
                            3 => instr_set.insert(op, "LBD".to_string()),
                            4 => instr_set.insert(op, "LBE".to_string()),
                            5 => instr_set.insert(op, "LBH".to_string()),
                            6 => instr_set.insert(op, "LBL".to_string()),
                            _ => continue
                        };
                    } else if reg1 == 208 {
                        match reg2 {
                            0 => instr_set.insert(op, "LCA".to_string()),
                            1 => instr_set.insert(op, "LCB".to_string()),
                            3 => instr_set.insert(op, "LCD".to_string()),
                            4 => instr_set.insert(op, "LCE".to_string()),
                            5 => instr_set.insert(op, "LCH".to_string()),
                            6 => instr_set.insert(op, "LCL".to_string()),
                            _ => continue
                        };
                    } else if reg1 == 216 {
                        match reg2 {
                            0 => instr_set.insert(op, "LDA".to_string()),
                            1 => instr_set.insert(op, "LDB".to_string()),
                            2 => instr_set.insert(op, "LDC".to_string()),
                            4 => instr_set.insert(op, "LDE".to_string()),
                            5 => instr_set.insert(op, "LDH".to_string()),
                            6 => instr_set.insert(op, "LDL".to_string()),
                            _ => continue
                        };
                    } else if reg1 == 224 {
                        match reg2 {
                            0 => instr_set.insert(op, "LEA".to_string()),
                            1 => instr_set.insert(op, "LEB".to_string()),
                            2 => instr_set.insert(op, "LEC".to_string()),
                            3 => instr_set.insert(op, "LED".to_string()),
                            5 => instr_set.insert(op, "LEH".to_string()),
                            6 => instr_set.insert(op, "LEL".to_string()),
                            _ => continue
                        };
                    } else if reg1 == 232 {
                        match reg2 {
                            0 => instr_set.insert(op, "LHA".to_string()),
                            1 => instr_set.insert(op, "LHB".to_string()),
                            2 => instr_set.insert(op, "LHC".to_string()),
                            3 => instr_set.insert(op, "LHD".to_string()),
                            4 => instr_set.insert(op, "LHE".to_string()),
                            6 => instr_set.insert(op, "LHL".to_string()),
                            _ => continue
                        };
                    } else if reg1 == 240 {
                        match reg2 {
                            0 => instr_set.insert(op, "LLA".to_string()),
                            1 => instr_set.insert(op, "LLB".to_string()),
                            2 => instr_set.insert(op, "LLC".to_string()),
                            3 => instr_set.insert(op, "LLD".to_string()),
                            4 => instr_set.insert(op, "LLE".to_string()),
                            5 => instr_set.insert(op, "LLH".to_string()),
                            _ => continue
                        };
                    }
                }
                
            }
        }
        instr_set
    }

    pub fn time_instr() -> [Vec<String>; 3] {
        let one_cycle_instrs: Vec<String> = vec![
            "LAB".to_string(),"LAC".to_string(),"LAD".to_string(),"LAE".to_string(),"LAH".to_string(),"LAL".to_string(),
            "LBA".to_string(),"LBC".to_string(),"LBD".to_string(),"LBE".to_string(),"LBH".to_string(),"LBL".to_string(),
            "LCA".to_string(),"LCB".to_string(),"LCD".to_string(),"LCE".to_string(),"LCH".to_string(),"LCL".to_string(),
            "LDA".to_string(),"LDB".to_string(),"LDC".to_string(),"LDE".to_string(),"LDH".to_string(),"LDL".to_string(),
            "LEA".to_string(),"LEB".to_string(),"LEC".to_string(),"LED".to_string(),"LEH".to_string(),"LEL".to_string(),
            "LHA".to_string(),"LHB".to_string(),"LHC".to_string(),"LHD".to_string(),"LHE".to_string(),"LHL".to_string(),
            "LLA".to_string(),"LLB".to_string(),"LLC".to_string(),"LLD".to_string(),"LLE".to_string(),"LLH".to_string(),
            "NOP".to_string()
        ];
        let two_cycle_instrs: Vec<String> = vec!["LMI".to_string()];
        let three_cycle_instrs: Vec<String> = vec!["CAL".to_string()];
        let instrs: [Vec<String>; 3] = [one_cycle_instrs, two_cycle_instrs, three_cycle_instrs];
        instrs
    }
    
    pub fn length_instr() -> [Vec<String>; 3] {
        let one_byte_instrs: Vec<String> = vec![
            "LAB".to_string(),"LAC".to_string(),"LAD".to_string(),"LAE".to_string(),"LAH".to_string(),"LAL".to_string(),
            "LBA".to_string(),"LBC".to_string(),"LBD".to_string(),"LBE".to_string(),"LBH".to_string(),"LBL".to_string(),
            "LCA".to_string(),"LCB".to_string(),"LCD".to_string(),"LCE".to_string(),"LCH".to_string(),"LCL".to_string(),
            "LDA".to_string(),"LDB".to_string(),"LDC".to_string(),"LDE".to_string(),"LDH".to_string(),"LDL".to_string(),
            "LEA".to_string(),"LEB".to_string(),"LEC".to_string(),"LED".to_string(),"LEH".to_string(),"LEL".to_string(),
            "LHA".to_string(),"LHB".to_string(),"LHC".to_string(),"LHD".to_string(),"LHE".to_string(),"LHL".to_string(),
            "LLA".to_string(),"LLB".to_string(),"LLC".to_string(),"LLD".to_string(),"LLE".to_string(),"LLH".to_string(),
            "NOP".to_string()
        ];
        let two_byte_instrs: Vec<String> = vec!["LMI".to_string()];
        let three_byte_instrs: Vec<String> = vec!["JMP".to_string()];
        let instrs: [Vec<String>; 3] = [one_byte_instrs, two_byte_instrs, three_byte_instrs];
        instrs
    }
    
    pub fn type_instr() -> [Vec<String>; 2] {
        let index_register_instrs: Vec<String> = vec![
            "LAB".to_string(),"LAC".to_string(),"LAD".to_string(),"LAE".to_string(),"LAH".to_string(),"LAL".to_string(),
            "LBA".to_string(),"LBC".to_string(),"LBD".to_string(),"LBE".to_string(),"LBH".to_string(),"LBL".to_string(),
            "LCA".to_string(),"LCB".to_string(),"LCD".to_string(),"LCE".to_string(),"LCH".to_string(),"LCL".to_string(),
            "LDA".to_string(),"LDB".to_string(),"LDC".to_string(),"LDE".to_string(),"LDH".to_string(),"LDL".to_string(),
            "LEA".to_string(),"LEB".to_string(),"LEC".to_string(),"LED".to_string(),"LEH".to_string(),"LEL".to_string(),
            "LHA".to_string(),"LHB".to_string(),"LHC".to_string(),"LHD".to_string(),"LHE".to_string(),"LHL".to_string(),
            "LLA".to_string(),"LLB".to_string(),"LLC".to_string(),"LLD".to_string(),"LLE".to_string(),"LLH".to_string()
        ];
        let machine_instr: Vec<String> = vec!["NOP".to_string(), "HLT".to_string()];
        let instrs: [Vec<String>; 2] = [index_register_instrs, machine_instr];
        instrs
    }
}