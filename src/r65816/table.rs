// code adapted from bsnes-mercury
// https://github.com/libretro/bsnes-mercury

use std::mem::{self, MaybeUninit};

use crate::r65816::{R65816, R65816Trait};

impl<T: R65816Trait> R65816<T> {
    #[allow(non_upper_case_globals)]
    const table_EM: usize = 0;
    #[allow(non_upper_case_globals)]
    const table_MX: usize = 256;
    #[allow(non_upper_case_globals)]
    const table_Mx: usize = 512;
    #[allow(non_upper_case_globals)]
    const table_mX: usize = 768;
    #[allow(non_upper_case_globals)]
    const table_mx: usize = 1024;

    #[inline(always)]
    fn op_m(
        op_table: &mut [MaybeUninit<fn(&mut Self)>; 5 * 256],
        id: usize,
        fn_b: fn(&mut Self),
        fn_w: fn(&mut Self),
    ) {
        op_table[Self::table_EM + id] = MaybeUninit::new(fn_b);
        op_table[Self::table_MX + id] = MaybeUninit::new(fn_b);
        op_table[Self::table_Mx + id] = MaybeUninit::new(fn_b);
        op_table[Self::table_mX + id] = MaybeUninit::new(fn_w);
        op_table[Self::table_mx + id] = MaybeUninit::new(fn_w);
    }

    #[inline(always)]
    fn op_x(
        op_table: &mut [MaybeUninit<fn(&mut Self)>; 5 * 256],
        id: usize,
        fn_b: fn(&mut Self),
        fn_w: fn(&mut Self),
    ) {
        op_table[Self::table_EM + id] = MaybeUninit::new(fn_b);
        op_table[Self::table_MX + id] = MaybeUninit::new(fn_b);
        op_table[Self::table_mX + id] = MaybeUninit::new(fn_b);
        op_table[Self::table_Mx + id] = MaybeUninit::new(fn_w);
        op_table[Self::table_mx + id] = MaybeUninit::new(fn_w);
    }

    pub fn initialize_opcode_table() -> [fn(&mut Self); 5 * 256] {
        const A: usize = 0;
        const X: usize = 1;
        const Y: usize = 2;
        const Z: usize = 3;

        let mut op_table = [MaybeUninit::uninit(); 5 * 256];
        Self::op_m(
            &mut op_table,
            0x01,
            |this| {
                this.op_read_idpx_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_idpx_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x03,
            |this| {
                this.op_read_sr_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_sr_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x05,
            |this| {
                this.op_read_dp_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_dp_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x07,
            |this| {
                this.op_read_ildp_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_ildp_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x09,
            |this| {
                this.op_read_const_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_const_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x0d,
            |this| {
                this.op_read_addr_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_addr_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x0f,
            |this| {
                this.op_read_long_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_long_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x11,
            |this| {
                this.op_read_idpy_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_idpy_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x12,
            |this| {
                this.op_read_idp_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_idp_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x13,
            |this| {
                this.op_read_isry_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_isry_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x15,
            |this| {
                this.op_read_dpr_b::<X>();
                this.op_ora_b();
            },
            |this| {
                this.op_read_dpr_w::<X>();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x17,
            |this| {
                this.op_read_ildpy_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_ildpy_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x19,
            |this| {
                this.op_read_addry_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_addry_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x1d,
            |this| {
                this.op_read_addrx_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_addrx_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x1f,
            |this| {
                this.op_read_longx_b();
                this.op_ora_b();
            },
            |this| {
                this.op_read_longx_w();
                this.op_ora_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x21,
            |this| {
                this.op_read_idpx_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_idpx_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x23,
            |this| {
                this.op_read_sr_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_sr_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x25,
            |this| {
                this.op_read_dp_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_dp_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x27,
            |this| {
                this.op_read_ildp_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_ildp_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x29,
            |this| {
                this.op_read_const_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_const_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x2d,
            |this| {
                this.op_read_addr_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_addr_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x2f,
            |this| {
                this.op_read_long_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_long_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x31,
            |this| {
                this.op_read_idpy_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_idpy_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x32,
            |this| {
                this.op_read_idp_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_idp_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x33,
            |this| {
                this.op_read_isry_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_isry_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x35,
            |this| {
                this.op_read_dpr_b::<X>();
                this.op_and_b();
            },
            |this| {
                this.op_read_dpr_w::<X>();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x37,
            |this| {
                this.op_read_ildpy_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_ildpy_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x39,
            |this| {
                this.op_read_addry_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_addry_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x3d,
            |this| {
                this.op_read_addrx_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_addrx_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x3f,
            |this| {
                this.op_read_longx_b();
                this.op_and_b();
            },
            |this| {
                this.op_read_longx_w();
                this.op_and_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x41,
            |this| {
                this.op_read_idpx_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_idpx_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x43,
            |this| {
                this.op_read_sr_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_sr_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x45,
            |this| {
                this.op_read_dp_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_dp_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x47,
            |this| {
                this.op_read_ildp_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_ildp_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x49,
            |this| {
                this.op_read_const_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_const_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x4d,
            |this| {
                this.op_read_addr_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_addr_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x4f,
            |this| {
                this.op_read_long_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_long_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x51,
            |this| {
                this.op_read_idpy_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_idpy_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x52,
            |this| {
                this.op_read_idp_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_idp_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x53,
            |this| {
                this.op_read_isry_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_isry_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x55,
            |this| {
                this.op_read_dpr_b::<X>();
                this.op_eor_b();
            },
            |this| {
                this.op_read_dpr_w::<X>();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x57,
            |this| {
                this.op_read_ildpy_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_ildpy_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x59,
            |this| {
                this.op_read_addry_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_addry_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x5d,
            |this| {
                this.op_read_addrx_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_addrx_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x5f,
            |this| {
                this.op_read_longx_b();
                this.op_eor_b();
            },
            |this| {
                this.op_read_longx_w();
                this.op_eor_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x61,
            |this| {
                this.op_read_idpx_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_idpx_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x63,
            |this| {
                this.op_read_sr_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_sr_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x65,
            |this| {
                this.op_read_dp_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_dp_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x67,
            |this| {
                this.op_read_ildp_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_ildp_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x69,
            |this| {
                this.op_read_const_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_const_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x6d,
            |this| {
                this.op_read_addr_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_addr_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x6f,
            |this| {
                this.op_read_long_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_long_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x71,
            |this| {
                this.op_read_idpy_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_idpy_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x72,
            |this| {
                this.op_read_idp_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_idp_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x73,
            |this| {
                this.op_read_isry_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_isry_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x75,
            |this| {
                this.op_read_dpr_b::<X>();
                this.op_adc_b();
            },
            |this| {
                this.op_read_dpr_w::<X>();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x77,
            |this| {
                this.op_read_ildpy_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_ildpy_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x79,
            |this| {
                this.op_read_addry_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_addry_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x7d,
            |this| {
                this.op_read_addrx_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_addrx_w();
                this.op_adc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0x7f,
            |this| {
                this.op_read_longx_b();
                this.op_adc_b();
            },
            |this| {
                this.op_read_longx_w();
                this.op_adc_w();
            },
        );
        Self::op_x(
            &mut op_table,
            0x8c,
            Self::op_write_addr_b::<Y>,
            Self::op_write_addr_w::<Y>,
        );
        Self::op_m(
            &mut op_table,
            0x8d,
            Self::op_write_addr_b::<A>,
            Self::op_write_addr_w::<A>,
        );
        Self::op_x(
            &mut op_table,
            0x8e,
            Self::op_write_addr_b::<X>,
            Self::op_write_addr_w::<X>,
        );
        Self::op_m(
            &mut op_table,
            0x8f,
            Self::op_write_longr_b::<Z>,
            Self::op_write_longr_w::<Z>,
        );
        Self::op_m(
            &mut op_table,
            0x99,
            Self::op_write_addrr_b::<A, Y>,
            Self::op_write_addrr_w::<A, Y>,
        );
        Self::op_m(
            &mut op_table,
            0x9c,
            Self::op_write_addr_b::<Z>,
            Self::op_write_addr_w::<Z>,
        );
        Self::op_m(
            &mut op_table,
            0x9d,
            Self::op_write_addrr_b::<A, X>,
            Self::op_write_addrr_w::<A, X>,
        );
        Self::op_m(
            &mut op_table,
            0x9e,
            Self::op_write_addrr_b::<Z, X>,
            Self::op_write_addrr_w::<Z, X>,
        );
        Self::op_m(
            &mut op_table,
            0x9f,
            Self::op_write_longr_b::<Z>,
            Self::op_write_longr_w::<Z>,
        );
        Self::op_m(
            &mut op_table,
            0xa1,
            |this| {
                this.op_read_idpx_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_idpx_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xa3,
            |this| {
                this.op_read_sr_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_sr_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xa5,
            |this| {
                this.op_read_dp_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_dp_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xa7,
            |this| {
                this.op_read_ildp_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_ildp_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xa9,
            |this| {
                this.op_read_const_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_const_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xad,
            |this| {
                this.op_read_addr_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_addr_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xaf,
            |this| {
                this.op_read_long_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_long_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xb1,
            |this| {
                this.op_read_idpy_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_idpy_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xb2,
            |this| {
                this.op_read_idp_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_idp_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xb3,
            |this| {
                this.op_read_isry_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_isry_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xb5,
            |this| {
                this.op_read_dpr_b::<X>();
                this.op_lda_b();
            },
            |this| {
                this.op_read_dpr_w::<X>();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xb7,
            |this| {
                this.op_read_ildpy_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_ildpy_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xb9,
            |this| {
                this.op_read_addry_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_addry_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xbd,
            |this| {
                this.op_read_addrx_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_addrx_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xbf,
            |this| {
                this.op_read_longx_b();
                this.op_lda_b();
            },
            |this| {
                this.op_read_longx_w();
                this.op_lda_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xc1,
            |this| {
                this.op_read_idpx_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_idpx_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xc3,
            |this| {
                this.op_read_sr_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_sr_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xc5,
            |this| {
                this.op_read_dp_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_dp_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xc7,
            |this| {
                this.op_read_ildp_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_ildp_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xc9,
            |this| {
                this.op_read_const_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_const_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xcd,
            |this| {
                this.op_read_addr_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_addr_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xcf,
            |this| {
                this.op_read_long_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_long_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xd1,
            |this| {
                this.op_read_idpy_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_idpy_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xd2,
            |this| {
                this.op_read_idp_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_idp_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xd3,
            |this| {
                this.op_read_isry_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_isry_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xd5,
            |this| {
                this.op_read_dpr_b::<X>();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_dpr_w::<X>();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xd7,
            |this| {
                this.op_read_ildpy_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_ildpy_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xd9,
            |this| {
                this.op_read_addry_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_addry_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xdd,
            |this| {
                this.op_read_addrx_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_addrx_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xdf,
            |this| {
                this.op_read_longx_b();
                this.op_cmp_b();
            },
            |this| {
                this.op_read_longx_w();
                this.op_cmp_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xe1,
            |this| {
                this.op_read_idpx_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_idpx_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xe3,
            |this| {
                this.op_read_sr_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_sr_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xe5,
            |this| {
                this.op_read_dp_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_dp_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xe7,
            |this| {
                this.op_read_ildp_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_ildp_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xe9,
            |this| {
                this.op_read_const_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_const_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xed,
            |this| {
                this.op_read_addr_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_addr_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xef,
            |this| {
                this.op_read_long_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_long_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xf1,
            |this| {
                this.op_read_idpy_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_idpy_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xf2,
            |this| {
                this.op_read_idp_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_idp_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xf3,
            |this| {
                this.op_read_isry_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_isry_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xf5,
            |this| {
                this.op_read_dpr_b::<X>();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_dpr_w::<X>();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xf7,
            |this| {
                this.op_read_ildpy_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_ildpy_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xf9,
            |this| {
                this.op_read_addry_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_addry_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xfd,
            |this| {
                this.op_read_addrx_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_addrx_w();
                this.op_sbc_w();
            },
        );
        Self::op_m(
            &mut op_table,
            0xff,
            |this| {
                this.op_read_longx_b();
                this.op_sbc_b();
            },
            |this| {
                this.op_read_longx_w();
                this.op_sbc_w();
            },
        );

        // assume_init
        unsafe { mem::transmute(op_table) }
    }
}
