/*//	Copyright 2023 Karl Vincent Pierre Bertin
////
////	Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:
////
////	1.	Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
////
////	2.	Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
////
////	3.	Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.
////
*///	THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![allow(non_snake_case)]

extern crate libm;
extern crate rand;

use libm::pow;
use libm::floor;
//use num_traits::float;
//use rand::random;
use crate::bcg7tid::l_n;

pub fn R_IDNo() -> ( u16 /* TID */, u16 /* SID */, i16 /* TSV */, i8 /* TRV */, i16 /* G7SID */, i32 /* G7TID */ )
{
	let maximum_16bitunsigned: u16;                  // 16-bit unsigned maximum value: 2**16 - 1 == 65535
	let integer_TID: u16;                            // generation I trainer ID number (TID): 0 <= i_TID <= 2**16 - 1
	let integer_SID: u16;                            // generation III secret ID number (SID): 0 <= i_SID <= 2**16 - 1
	let result: u16;                                 // i_TID ^ i_SID
	let integer_TSV: i16;                            // trainer shiny value (TSV): 0 <= i_TSV <= ( min( 0x0, 0xFFFF ) ^ max( 0x0, 0xFFFF ) ) >> 4 == 4095, i_TSV = ( i_TID ^ i_SID ) >> 4
	let integer_TRV: i8;                             // trainer residual value (TRV): 0 <= i_TRV <= ( min( 0x0, 0xFFFF ) ^ max( 0x0, 0xFFFF ) ) & 0xF == 15, i_TRV = ( i_TID ^ i_SID ) & 0xF
	let integer_IDNo: u32;                           // generation VII ID number (G7ID, 'IDNo'): 0 <= i_IDNo <= 2**32 - 1 == 4294967295, i_G7ID = i_TID + ( 2**16 * i_SID )
	let million: u32;                                // 'mask': 10**6 == 1000000
	let integer_G7TID: i32;                          // generation VII trainer ID number (G7TID): 0 <= i_G7TID <= 999999, i_G7TID = i_G7ID % 10**6
	let radix: i8;                                   // positional numeral system: decimal system ( radix = 10 )
	let maximum_32bitunsigned: u32;                  // 32-bit unsigned maximum value: 2**32 - 1 == 4294967295
	let length_maximum: u8;                          // 32-bit unsigned maximum value length: for r = 10, l_max = floor( r - log_r( i_max ) + 1 ) == 10
	let length_G7TID: u8;                            // G7TID length: l_G7TID = 6 == l_max - l_G7SID
	let length_G7SID: u8;                            // G7SID length: l_G7SID = l_max - l_G7TID == 4
	let integer_G7SID: i16;                          // generation VII secret ID number (G7SID): 0 <= i_G7SID <= 4294, i_G7SID = floor( n / 10**( l_max - 6 ) )
	let tuple_IDNo: ( u16, u16, i16, i8, i16, i32 ); // ( i_TID, i_SID, i_TSV, i_TRV, i_G7SID, i_G7TID )

	maximum_16bitunsigned = ( pow( 2f64, 16f64 ) - 1f64 ) as u16;
	integer_SID = rand::random::<u16>() % maximum_16bitunsigned;
	integer_TID = rand::random::<u16>() % maximum_16bitunsigned;
	result = integer_TID ^ integer_SID;
	integer_TSV = ( result >> 4 ) as i16;
	integer_TRV = ( result & 0xF ) as i8;
	million = 1_000_000u32; //million = pow( 10f64, 6f64 ) as i32;
	maximum_32bitunsigned = ( pow( 2f64, 32f64 ) - 1f64 ) as u32;
	radix = 10i8;
	length_maximum = l_n::l_n( maximum_32bitunsigned, radix );
	length_G7TID = 6u8;
	length_G7SID = ( length_maximum - length_G7TID ) as u8;
	integer_IDNo = ( integer_TID as f64 + ( integer_SID as f64 * pow( 2f64, 16f64 ) ) ) as u32;
	integer_G7TID = ( integer_IDNo % million as u32 ) as i32;
	integer_G7SID = floor( integer_IDNo as f64 / pow( 10f64, ( length_maximum - length_G7SID ) as f64 ) ) as i16;
	tuple_IDNo = ( integer_TID, integer_SID, integer_TSV, integer_TRV, integer_G7SID, integer_G7TID );

	return tuple_IDNo;
}

/*//	R_IDNo.rs
////	CIDNo (IDNo Calculator)
////
////	Karl V. P. B. `kvpb`	AKA Karl Thomas George West `ktgw`
////	+33 A BB BB BB BB		+1 (DDD) DDD-DDDD
////	local-part@domain
////	https://x.com/ktgwkvpb
*///	https://github.com/kvpb
