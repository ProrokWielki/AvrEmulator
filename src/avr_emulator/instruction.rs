use crate::avr_emulator::memory::Memory;

mod adc;
mod add;
mod andi;
mod brbc;
mod brbs;
mod breq;
mod brge;
mod brlt;
mod brne;
mod bset;
mod cp;
mod cpc;
mod cpi;
mod eor;
mod i_in;
mod ld_z;
mod ldd_y;
mod ldi;
mod lds;
mod lpm_z_plus;
mod mov;
mod movw;
mod nop;
mod out;
mod pop;
mod push;
mod rcall;
mod ret;
mod reti;
mod rjmp;
mod sbc;
mod sbci;
mod sbiw;
mod sbr;
mod st_z;
mod std_y;
mod sts;
mod sub;
mod subi;

pub trait Instruction {
    fn process(&self, memory: &mut Memory) -> ();
    fn str(&self) -> String;

    fn get_instruction_codes() -> Vec<u16>
    where
        Self: Sized;
    fn get_instruction_mask() -> u16
    where
        Self: Sized;

    fn eq(opcode: u16) -> bool
    where
        Self: Sized,
    {
        for instcruction_code in Self::get_instruction_codes() {
            if opcode & Self::get_instruction_mask() == instcruction_code {
                return true;
            }
        }
        false
    }

    fn extend(value: i16, orginal_length: u8) -> i16
    where
        Self: Sized,
    {
        let mask = 1 << (orginal_length - 1);
        (value ^ mask) - mask
    }
}

//TODO: refactor
pub fn get_instruction(opcode: u16) -> Option<Box<dyn Instruction>> {
    if nop::NOP::eq(opcode) {
        return Some(Box::new(nop::NOP::new(opcode)));
    }
    if ret::RET::eq(opcode) {
        return Some(Box::new(ret::RET::new(opcode)));
    }
    if rjmp::RJMP::eq(opcode) {
        return Some(Box::new(rjmp::RJMP::new(opcode)));
    }
    if push::PUSH::eq(opcode) {
        return Some(Box::new(push::PUSH::new(opcode)));
    }
    if eor::EOR::eq(opcode) {
        return Some(Box::new(eor::EOR::new(opcode)));
    }
    if i_in::IN::eq(opcode) {
        return Some(Box::new(i_in::IN::new(opcode)));
    }
    if out::OUT::eq(opcode) {
        return Some(Box::new(out::OUT::new(opcode)));
    }
    if ldi::LDI::eq(opcode) {
        return Some(Box::new(ldi::LDI::new(opcode)));
    }
    if rcall::RCALL::eq(opcode) {
        return Some(Box::new(rcall::RCALL::new(opcode)));
    }
    if std_y::STDY::eq(opcode) {
        return Some(Box::new(std_y::STDY::new(opcode)));
    }
    if ldd_y::LDDY::eq(opcode) {
        return Some(Box::new(ldd_y::LDDY::new(opcode)));
    }
    if movw::MOVW::eq(opcode) {
        return Some(Box::new(movw::MOVW::new(opcode)));
    }
    if mov::MOV::eq(opcode) {
        return Some(Box::new(mov::MOV::new(opcode)));
    }
    if subi::SUBI::eq(opcode) {
        return Some(Box::new(subi::SUBI::new(opcode)));
    }
    if sbci::SBCI::eq(opcode) {
        return Some(Box::new(sbci::SBCI::new(opcode)));
    }
    if cp::CP::eq(opcode) {
        return Some(Box::new(cp::CP::new(opcode)));
    }
    if cpc::CPC::eq(opcode) {
        return Some(Box::new(cpc::CPC::new(opcode)));
    }
    if brlt::BRLT::eq(opcode) {
        return Some(Box::new(brlt::BRLT::new(opcode)));
    }
    if sbc::SBC::eq(opcode) {
        return Some(Box::new(sbc::SBC::new(opcode)));
    }
    if cpi::CPI::eq(opcode) {
        return Some(Box::new(cpi::CPI::new(opcode)));
    }
    if brge::BRGE::eq(opcode) {
        return Some(Box::new(brge::BRGE::new(opcode)));
    }
    if pop::POP::eq(opcode) {
        return Some(Box::new(pop::POP::new(opcode)));
    }
    if sbiw::SBIW::eq(opcode) {
        return Some(Box::new(sbiw::SBIW::new(opcode)));
    }
    if breq::BREQ::eq(opcode) {
        return Some(Box::new(breq::BREQ::new(opcode)));
    }
    if brne::BRNE::eq(opcode) {
        return Some(Box::new(brne::BRNE::new(opcode)));
    }
    if ld_z::LDZ::eq(opcode) {
        return Some(Box::new(ld_z::LDZ::new(opcode)));
    }
    if sbr::SBR::eq(opcode) {
        return Some(Box::new(sbr::SBR::new(opcode)));
    }
    if st_z::STZ::eq(opcode) {
        return Some(Box::new(st_z::STZ::new(opcode)));
    }
    if bset::BSET::eq(opcode) {
        return Some(Box::new(bset::BSET::new(opcode)));
    }
    if andi::ANDI::eq(opcode) {
        return Some(Box::new(andi::ANDI::new(opcode)));
    }
    if brbs::BRBS::eq(opcode) {
        return Some(Box::new(brbs::BRBS::new(opcode)));
    }
    if reti::RETI::eq(opcode) {
        return Some(Box::new(reti::RETI::new(opcode)));
    }
    if add::ADD::eq(opcode) {
        return Some(Box::new(add::ADD::new(opcode)));
    }
    if adc::ADC::eq(opcode) {
        return Some(Box::new(adc::ADC::new(opcode)));
    }
    if lpm_z_plus::LpmZPlus::eq(opcode) {
        return Some(Box::new(lpm_z_plus::LpmZPlus::new(opcode)));
    }
    if brbc::BRBC::eq(opcode) {
        return Some(Box::new(brbc::BRBC::new(opcode)));
    }
    if lds::LDS::eq(opcode) {
        return Some(Box::new(lds::LDS::new(opcode)));
    }
    if sts::STS::eq(opcode) {
        return Some(Box::new(sts::STS::new(opcode)));
    }
    if sub::SUB::eq(opcode) {
        return Some(Box::new(sub::SUB::new(opcode)));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockInstruction {}

    impl Instruction for MockInstruction {
        fn process(&self, _registers: &mut Memory) {}
        fn str(&self) -> String {
            return "mock".to_owned();
        }
        fn get_instruction_codes() -> Vec<u16> {
            vec![0xf0f0]
        }
        fn get_instruction_mask() -> u16 {
            0xfff0
        }
    }

    #[test]
    fn test_eq_returns_true_for_valid_instruction() {
        assert!(MockInstruction::eq(0xf0f0));
        assert!(MockInstruction::eq(0xf0ff));
    }

    #[test]
    fn test_eq_returns_false_for_invalid_instruction() {
        assert!(!MockInstruction::eq(0x0000));
        assert!(!MockInstruction::eq(0xf000));
        assert!(!MockInstruction::eq(0x00f0));
        assert!(!MockInstruction::eq(0xe0e0));
    }

    #[test]
    fn test_extend_return_correct_value_for_positive_integers() {
        assert_eq!(MockInstruction::extend(0x0001, 8), 1);
        assert_eq!(MockInstruction::extend(0x00ff, 9), 255);
        assert_eq!(MockInstruction::extend(0x0003, 4), 3);
    }

    #[test]
    fn test_extend_return_correct_value_for_negative_integers() {
        assert_eq!(MockInstruction::extend(0x0003, 2), -1);
        assert_eq!(MockInstruction::extend(0x00ff, 8), -1);
        assert_eq!(MockInstruction::extend(0x0100, 9), -256);
    }

    #[test]
    fn test_get_instruction_returns_none_for_invalid_opcode() {
        assert!(get_instruction(0xffff).is_none());
    }

    #[test]
    fn test_get_instruction_returns_nop_for_nop_opcode() {
        assert_eq!(get_instruction(0x0000).unwrap().str(), "nop");
    }

    #[test]
    fn test_get_instruction_returns_ret_for_ret_opcode() {
        assert_eq!(get_instruction(0b1001_0101_0000_1000).unwrap().str(), "ret");
    }

    #[test]
    fn test_get_instruction_returns_rjmp_for_rjmp_opcode() {
        assert_eq!(get_instruction(0xcfff).unwrap().str(), "rjmp -1");
    }

    #[test]
    fn test_get_instruction_returns_push_for_push_opcode() {
        let pushed_value = 7;
        assert_eq!(
            get_instruction(push::PUSH::get_instruction_codes()[0] | pushed_value << 4)
                .unwrap()
                .str(),
            format!("push r{}", pushed_value)
        );
    }

    #[test]
    fn test_get_instruction_returns_eor_for_eor_opcode() {
        assert_eq!(get_instruction(0x2443).unwrap().str(), "eor r4, r3");
    }

    #[test]
    fn test_get_instruction_returns_in_for_in_opcode() {
        assert_eq!(get_instruction(0xb000).unwrap().str(), "in r0, 0");
    }

    #[test]
    fn test_get_instruction_returns_out_for_out_opcode() {
        assert_eq!(get_instruction(0xb800).unwrap().str(), "out 0, r0");
    }

    #[test]
    fn test_get_instruction_returns_ldi_for_ldi_opcode() {
        assert_eq!(get_instruction(0xefff).unwrap().str(), "ldi r31, 255");
    }

    #[test]
    fn test_get_instruction_returns_rcall_for_rcall_opcode() {
        assert_eq!(get_instruction(0xdfff).unwrap().str(), "rcall -1");
    }

    #[test]
    fn test_get_instruction_returns_std_for_std_opcode() {
        assert_eq!(get_instruction(0x8a08).unwrap().str(), "std y+16, r0");
    }

    #[test]
    fn test_get_instruction_returns_ldd_for_ldd_opcode() {
        assert_eq!(get_instruction(0x8828).unwrap().str(), "ldd r2, y+16");
    }

    #[test]
    fn test_get_instruction_returns_movw_for_movw_opcode() {
        assert_eq!(get_instruction(0x0112).unwrap().str(), "movw r2, r4");
    }

    #[test]
    fn test_get_instruction_returns_subi_for_subi_opcode() {
        assert_eq!(get_instruction(0x5032).unwrap().str(), "subi r19, 2");
    }

    #[test]
    fn test_get_instruction_returns_sbci_for_sbci_opcode() {
        assert_eq!(get_instruction(0x4045).unwrap().str(), "sbci r20, 5");
    }

    #[test]
    fn test_get_instruction_returns_cp_for_cp_opcode() {
        assert_eq!(get_instruction(0x1456).unwrap().str(), "cp r5, r6");
    }

    #[test]
    fn test_get_instruction_returns_cpc_for_cpc_opcode() {
        assert_eq!(get_instruction(0x048c).unwrap().str(), "cpc r8, r12");
    }

    #[test]
    fn test_get_instruction_returns_brlt_for_brlt_opcode() {
        assert_eq!(get_instruction(0xf004).unwrap().str(), "brlt 0");
    }

    #[test]
    fn test_get_instruction_returns_sbc_for_sbc_opcode() {
        assert_eq!(get_instruction(0x089a).unwrap().str(), "sbc r9, r10");
    }

    #[test]
    fn test_get_instruction_returns_cpi_for_cpi_opcode() {
        assert_eq!(get_instruction(0x3012).unwrap().str(), "cpi r17, 2");
    }

    #[test]
    fn test_get_instruction_returns_brge_for_brge_opcode() {
        assert_eq!(get_instruction(0xf414).unwrap().str(), "brge 2");
    }

    #[test]
    fn test_get_instruction_returns_pop_for_pop_opcode() {
        assert_eq!(get_instruction(0x90ff).unwrap().str(), "pop r15");
    }

    #[test]
    fn test_get_instruction_returns_sbiw_for_sbiw_opcode() {
        assert_eq!(get_instruction(0x9700).unwrap().str(), "sbiw r25:r24, 0");
    }

    #[test]
    fn test_get_instruction_returns_breq_for_breq_opcode() {
        assert_eq!(get_instruction(0xf001).unwrap().str(), "breq 0");
    }

    #[test]
    fn test_get_instruction_returns_brne_for_brne_opcode() {
        assert_eq!(get_instruction(0xf401).unwrap().str(), "brne 0");
    }

    #[test]
    fn test_get_instruction_returns_ldz_for_ldz_opcode() {
        assert_eq!(get_instruction(0x8010).unwrap().str(), "ld r1, z");
    }

    #[test]
    fn test_get_instruction_returns_sbr_for_sbr_opcode() {
        assert_eq!(get_instruction(0x6000).unwrap().str(), "sbr r16, 0");
    }

    #[test]
    fn test_get_instruction_returns_stz_for_stz_opcode() {
        assert_eq!(get_instruction(0x8200).unwrap().str(), "st z, r0");
    }

    #[test]
    fn test_get_instruction_returns_bset_for_bset_opcode() {
        assert_eq!(get_instruction(0x9408).unwrap().str(), "bset 0");
    }

    #[test]
    fn test_get_instruction_returns_mov_for_mov_opcode() {
        assert_eq!(get_instruction(0x2c01).unwrap().str(), "mov r0, r1");
    }

    #[test]
    fn test_get_instruction_returns_andi_for_andi_opcode() {
        assert_eq!(get_instruction(0x7012).unwrap().str(), "andi r17, 2");
    }

    #[test]
    fn test_get_instruction_returns_brbs_for_brbs_opcode() {
        assert_eq!(get_instruction(0xf012).unwrap().str(), "brbs 2, 2");
    }

    #[test]
    fn test_get_instruction_returns_reti_for_reti_opcode() {
        assert_eq!(get_instruction(0x9518).unwrap().str(), "reti");
    }

    #[test]
    fn test_get_instruction_returns_add_for_add_opcode() {
        assert_eq!(get_instruction(0x0c12).unwrap().str(), "add r1, r2");
    }

    #[test]
    fn test_get_instruction_returns_adc_for_adc_opcode() {
        assert_eq!(get_instruction(0x1c12).unwrap().str(), "adc r1, r2");
    }

    #[test]
    fn test_get_instruction_returns_lpm_z_plus_for_lpm_z_plus_opcode() {
        assert_eq!(get_instruction(0x9005).unwrap().str(), "lpm r0, z+");
    }

    #[test]
    fn test_get_instruction_returns_brbc_for_brbc_opcode() {
        assert_eq!(get_instruction(0xf412).unwrap().str(), "brbc 2, 2");
    }

    #[test]
    fn test_get_instruction_returns_lds_for_lds_opcode() {
        assert_eq!(get_instruction(0x9000).unwrap().str(), "lds r0, 0");
    }

    #[test]
    fn test_get_instruction_returns_sts_for_sts_opcode() {
        assert_eq!(get_instruction(0x9200).unwrap().str(), "sts 0, r0");
    }

    #[test]
    fn test_get_instruction_returns_sub_for_sub_opcode() {
        assert_eq!(get_instruction(0x1812).unwrap().str(), "sub r1, r2");
    }
}

#[cfg(test)]
mod integraiton_test {
    use crate::avr_emulator::memory::Memory;

    use super::nop;
    use super::rcall;
    use super::ret;
    use super::Instruction;

    #[test]
    fn test_ret_after_rcall() {
        let return_pc = 345;
        let rcall_offset = 400;

        let rcall = rcall::RCALL::new(0xd000 | (rcall_offset as u16));
        let ret = ret::RET::new(0x9508);
        let nop = nop::NOP::new(0x0000);

        let mut memory = Memory::new(300, vec![]).unwrap();
        memory.set_pc(return_pc);
        memory.set_sp(50);

        rcall.process(&mut memory);
        assert_eq!(memory.get_pc(), return_pc + rcall_offset + 1);

        nop.process(&mut memory);
        assert_eq!(memory.get_pc(), return_pc + rcall_offset + 1 + 1);

        ret.process(&mut memory);
        assert_eq!(memory.get_pc(), return_pc + 1);
    }
}
