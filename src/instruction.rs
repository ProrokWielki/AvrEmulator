use crate::registers::Registers;

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
mod i_std_y;
mod ld_z;
mod ldd_y;
mod ldi;
mod movw;
mod nop;
mod out;
mod pop;
mod push;
mod rcall;
mod ret;
mod rjmp;
mod sbc;
mod sbci;
mod sbiw;
mod sbr;
mod st_z;
mod subi;

pub trait Instruction {
    fn process(&self, registers: &mut Registers) -> ();
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
    if i_std_y::STDY::eq(opcode) {
        return Some(Box::new(i_std_y::STDY::new(opcode)));
    }
    if ldd_y::LDDY::eq(opcode) {
        return Some(Box::new(ldd_y::LDDY::new(opcode)));
    }
    if movw::MOVW::eq(opcode) {
        return Some(Box::new(movw::MOVW::new(opcode)));
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

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockInstruction {}

    impl Instruction for MockInstruction {
        fn process(&self, _registers: &mut Registers) {}
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
    fn test_eq_returns_true_for_valid_instraction() {
        assert!(MockInstruction::eq(0xf0f0));
        assert!(MockInstruction::eq(0xf0ff));
    }

    #[test]
    fn test_eq_returns_false_for_invalid_instraction() {
        assert!(!MockInstruction::eq(0x0000));
        assert!(!MockInstruction::eq(0xf000));
        assert!(!MockInstruction::eq(0x00f0));
        assert!(!MockInstruction::eq(0xe0e0));
    }

    #[test]
    fn test_extend_return_correct_value_for_positiv_integers() {
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
    fn test_get_instruction_retunrs_none_for_invalid_opcode() {
        assert!(get_instruction(0xffff).is_none());
    }

    #[test]
    fn test_get_instruction_retunrs_nop_for_nop_opcode() {
        assert_eq!(get_instruction(0x0000).unwrap().str(), "nop");
    }

    #[test]
    fn test_get_instruction_retunrs_ret_for_ret_opcode() {
        assert_eq!(get_instruction(0b1001_0101_0000_1000).unwrap().str(), "ret");
    }

    #[test]
    fn test_get_instruction_retunrs_rjmp_for_rjmp_opcode() {
        assert_eq!(get_instruction(0xcfff).unwrap().str(), "rjmp -1");
    }

    #[test]
    fn test_get_instruction_retunrs_push_for_push_opcode() {
        let pushed_value = 7;
        assert_eq!(
            get_instruction(push::PUSH::get_instruction_codes()[0] | pushed_value << 4)
                .unwrap()
                .str(),
            format!("push r{}", pushed_value)
        );
    }

    #[test]
    fn test_get_instruction_retunrs_eor_for_eor_opcode() {
        assert_eq!(get_instruction(0x2443).unwrap().str(), "eor r4, r3");
    }

    #[test]
    fn test_get_instruction_retunrs_in_for_in_opcode() {
        assert_eq!(get_instruction(0xb000).unwrap().str(), "in r0, 0");
    }

    #[test]
    fn test_get_instruction_retunrs_out_for_out_opcode() {
        assert_eq!(get_instruction(0xb800).unwrap().str(), "out 0, r0");
    }

    #[test]
    fn test_get_instruction_retunrs_ldi_for_ldi_opcode() {
        assert_eq!(get_instruction(0xefff).unwrap().str(), "ldi r31, 255");
    }

    #[test]
    fn test_get_instruction_retunrs_rcall_for_rcall_opcode() {
        assert_eq!(get_instruction(0xdfff).unwrap().str(), "rcall -1");
    }

    #[test]
    fn test_get_instruction_retunrs_std_for_std_opcode() {
        assert_eq!(get_instruction(0x8a08).unwrap().str(), "std y+16, r0");
    }

    #[test]
    fn test_get_instruction_retunrs_ldd_for_ldd_opcode() {
        assert_eq!(get_instruction(0x8828).unwrap().str(), "ldd r2, y+16");
    }

    #[test]
    fn test_get_instruction_retunrs_movw_for_movw_opcode() {
        assert_eq!(get_instruction(0x0112).unwrap().str(), "movw r2, r4");
    }

    #[test]
    fn test_get_instruction_retunrs_subi_for_subi_opcode() {
        assert_eq!(get_instruction(0x5032).unwrap().str(), "subi r19, 2");
    }

    #[test]
    fn test_get_instruction_retunrs_sbci_for_sbci_opcode() {
        assert_eq!(get_instruction(0x4045).unwrap().str(), "sbci r20, 5");
    }

    #[test]
    fn test_get_instruction_retunrs_cp_for_cp_opcode() {
        assert_eq!(get_instruction(0x1456).unwrap().str(), "cp r5, r6");
    }

    #[test]
    fn test_get_instruction_retunrs_cpc_for_cpc_opcode() {
        assert_eq!(get_instruction(0x048c).unwrap().str(), "cpc r8, r12");
    }

    #[test]
    fn test_get_instruction_retunrs_brlt_for_brlt_opcode() {
        assert_eq!(get_instruction(0xf004).unwrap().str(), "brlt 0");
    }

    #[test]
    fn test_get_instruction_retunrs_sbc_for_sbc_opcode() {
        assert_eq!(get_instruction(0x089a).unwrap().str(), "sbc r9, r10");
    }

    #[test]
    fn test_get_instruction_retunrs_cpi_for_cpi_opcode() {
        assert_eq!(get_instruction(0x3012).unwrap().str(), "cpi r17, 2");
    }

    #[test]
    fn test_get_instruction_retunrs_brge_for_brge_opcode() {
        assert_eq!(get_instruction(0xf414).unwrap().str(), "brge 2");
    }

    #[test]
    fn test_get_instruction_retunrs_pop_for_pop_opcode() {
        assert_eq!(get_instruction(0x90ff).unwrap().str(), "pop r15");
    }

    #[test]
    fn test_get_instruction_retunrs_sbiw_for_sbiw_opcode() {
        assert_eq!(get_instruction(0x9700).unwrap().str(), "sbiw r25:r24, 0");
    }

    #[test]
    fn test_get_instruction_retunrs_breq_for_breq_opcode() {
        assert_eq!(get_instruction(0xf001).unwrap().str(), "breq 0");
    }

    #[test]
    fn test_get_instruction_retunrs_brne_for_brne_opcode() {
        assert_eq!(get_instruction(0xf401).unwrap().str(), "brne 0");
    }

    #[test]
    fn test_get_instruction_retunrs_ldz_for_ldz_opcode() {
        assert_eq!(get_instruction(0x8010).unwrap().str(), "ld r1, z");
    }

    #[test]
    fn test_get_instruction_retunrs_sbr_for_sbr_opcode() {
        assert_eq!(get_instruction(0x6000).unwrap().str(), "sbr r16, 0");
    }

    #[test]
    fn test_get_instruction_retunrs_stz_for_stz_opcode() {
        assert_eq!(get_instruction(0x8200).unwrap().str(), "st z, r0");
    }

    #[test]
    fn test_get_instruction_retunrs_bset_for_bset_opcode() {
        assert_eq!(get_instruction(0x9408).unwrap().str(), "bset 0");
    }
}
