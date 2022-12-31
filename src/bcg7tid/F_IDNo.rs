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

extern crate libm;
extern crate rand;

use crate::bcg7tid::F_IDNo::libm::*; //use libm::*;
use crate::bcg7tid::F_IDNo::rand::*; //use rand::*;
use crate::bcg7tid::l_n::l_n;

pub fn F_IDNo( integer_G7TID_target: i32 ) -> ( i16 /* G7SID */, i32 /* G7TID */, i8 /* TRV */, i16 /* TSV */, u16 /* SID */, u16 /* TID */ )
{
	let maximum_16bitunsigned: u16;                  // 16-bit unsigned maximum value: 2**16 - 1 == 65535
	let mut integer_TID: u16;                        // generation I trainer ID number (TID): 0 <= i_TID <= 2**16 - 1
	let mut integer_SID: u16;                        // generation III secret ID number (SID): 0 <= i_SID <= 2**16 - 1
	let mut result: u16;                             // i_TID ^ i_SID
	let mut integer_TSV: i16;                        // trainer shiny value (TSV): 0 <= i_TSV <= ( min( 0x0, 0xFFFF ) ^ max( 0x0, 0xFFFF ) ) >> 4 == 4095, i_TSV = ( i_TID ^ i_SID ) >> 4
	let mut integer_TRV: i8;                         // trainer residual value (TRV): 0 <= i_TRV <= ( min( 0x0, 0xFFFF ) ^ max( 0x0, 0xFFFF ) ) & 0xF == 15, i_TRV = ( i_TID ^ i_SID ) & 0xF
	let mut integer_IDNo: u32;                       // generation VII ID number (G7ID, 'IDNo'): 0 <= i_IDNo <= 2**32 - 1 == 4294967295, i_G7ID = i_TID + ( 2**16 * i_SID )
	let million: i32;                                // one million ('mask'): 10**6 == 1000000
	let mut integer_G7TID: i32;                      // generation VII trainer ID number (G7TID): 0 <= i_G7TID <= 999999, i_G7TID = i_G7ID % 10**6
	let radix: i8;                                   // positional numeral system: decimal system ( radix = 10 )
	let maximum_32bitunsigned: u32;                  // 32-bit unsigned maximum value: 2**32 - 1 == 4294967295
	let mut length_maximum: u8;                      // 32-bit unsigned maximum value length: for r = 10, l_max = floor( r - log_r( i_max ) + 1 ) == 10
	let length_G7TID: u8;                            // G7TID length: l_G7TID = 6 == l_max - l_G7SID
	let mut length_G7SID: u8;                        // G7SID length: l_G7SID = l_max - l_G7TID == 4
	let mut integer_G7SID: i16;                      // generation VII secret ID number (G7SID): 0 <= i_G7SID <= 4294, i_G7SID = floor( n / 10**( l_max - 6 ) )
	let mut boolean_flag: bool;
	let tuple_IDNo: ( i16, i32, i8, i16, u16, u16 ); // ( i_G7SID, i_G7TID, i_TRV, i_TSV, i_SID, i_TID )

	maximum_16bitunsigned = u16::MAX /*( pow( 2, 16 ) - 1 ) as u16*/; // My code exhaustively searches for a G7TID. The likes of POW will only slow it down.
	million = pow( 10f64, 6f64 ) as i32;
	length_G7TID = 6;
	radix = 10;
	maximum_32bitunsigned = u32::MAX /*( pow( 2, 32 ) - 1 ) as u32*/;
	length_maximum = l_n( maximum_32bitunsigned, radix ) /*10*/;
	integer_TID = rand::thread_rng().gen_range( 0..maximum_16bitunsigned );
	boolean_flag = false;
	while boolean_flag == false
	{
		integer_SID = 0;
		while integer_SID < maximum_16bitunsigned && boolean_flag == false
		{
			integer_IDNo = integer_TID as u32 + ( ( maximum_16bitunsigned as u32 + 1 ) * integer_SID as u32 ) as u32;
			integer_G7TID = ( integer_IDNo % million as u32 ) as i32;
			if integer_G7TID == integer_G7TID_target
			{
				length_G7SID = length_maximum - length_G7TID;
				integer_G7SID = floor( ( integer_IDNo as f64 / pow( 10f64, <u8 as Into<f64>>::into( ( length_maximum - length_G7SID ) ) ) ).into() ) as i16;
				result = ( integer_TID /*( integer_IDNo >> 16 )*/ ^ integer_SID /*( integer_IDNo & 0xFFFF )*/ ) as u16;
				integer_TRV = ( result /*( integer_TID ^ integer_SID )*/ & 0xF ) as i8;
				integer_TSV = ( result /*( integer_TID ^ integer_SID )*/ >> 4 ) as i16;
				tuple_IDNo = ( integer_G7SID, integer_G7TID, integer_TRV, integer_TSV, integer_SID, integer_TID );
				boolean_flag = true; // Break out of this control flow statement.
				return tuple_IDNo; // Return.
			}
			integer_SID += 1;
		}
		integer_SID -= 1;
		integer_TID = rand::thread_rng().gen_range( 0..maximum_16bitunsigned ); // If I did not find a valid IDNo, reroll.
	} // I gotta rewrite that block, optimize my code.

	return ( 0i16, 0i32, 0i8, 0i16, 0u16, 0u16 ); // Because RustC expect a tuple ( i16, i32, i8, i16, u16, u16 ), at least return ( 0, 0, 0, 0, 0, 0 ), or else it will not compile this at all.
} // I must rewrite that function, optimize my code.

/*//	F_IDNo.rs
////	CIDNo (IDNo Calculator) //CG7ID (G7ID Calculator)
////
////	Karl V. P. B. `kvpb`	AKA Karl Thomas George West `ktgw`
////	+33 A BB BB BB BB		+1 (DDD) DDD-DDDD
////	local-part@domain
////	https://twitter.com/ktgwkvpb
*///	https://github.com/kvpb
