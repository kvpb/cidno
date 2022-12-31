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

use libm::log2;
use libm::log10;
use libm::floor;

fn l_n( number: u32, radix: i8 ) -> u8
{
	if radix != 10 || radix != 2
	{
		return 1;
	}

	let mut length: u8;

	length = 0;
	if radix == 10
	{
		length = floor( log10( number.into() ) + 1.0 ) as u8;
	}
	else if radix == 2
	{
		length = floor( log2( number.into() ) + 1.0 ) as u8;
	}

	return length;
}

/*//	l_n.rs
////	CIDNo (IDNo Calculator) //CG7ID (G7ID Calculator)
////
////	Karl V. P. B. `kvpb`	AKA Karl Thomas George West `ktgw`
////	+33 A BB BB BB BB		+1 (DDD) DDD-DDDD
////	local-part@domain
////	https://twitter.com/ktgwkvpb
*///	https://github.com/kvpb
