use super::*;
use proptest::prelude::*;

const TOTAL_PIXELS: usize = SCREEN_HEIGHT * SCREEN_WIDTH;

prop_compose! {
    fn arb_vm()(memory in any::<[u8; 4096]>(), 
    registers in any::<[u8; 16]>(), 
    stack in any::<[u16; 16]>(), 
    stack_pointer in any::<u8>(), 
    screen in any::<[u8; TOTAL_PIXELS]>(), 
    index_register in any::<u16>(), program_counter in any::<u16>(), 
    delay_timer in any::<u8>(), sound_timer in any::<u8>(), 
    key_state in any::<[bool; 16]>(), 
    blocked_on_key_press in any::<bool>()) -> VirtualMachine {
        VirtualMachine {
            memory,
            registers,
            stack,
            stack_pointer,
            screen,
            index_register,
            program_counter,
            delay_timer,
            sound_timer,
            key_state,
            blocked_on_key_press
        }
    }
}

proptest! {
    #[test]
    fn test_reset(mut vm in arb_vm()) {
        vm.reset();
        assert_eq!(vm, VirtualMachine::new());
    }
}
