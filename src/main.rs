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

#![allow(unsafe_code)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_comparisons)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(warnings)]

//extern crate cxx;
extern crate libm;
extern crate rand;
extern crate num_traits;
//extern crate cidno;
//extern crate cg7tid;
//extern crate bcg7tid;
//extern crate rcg7tid;

//#[repr(C)]
//extern "C"
//{
//	fn nbrlen( number: u32, radix: i8 ) -> u8;
//	fn f_IDNo/*IDNo_calculate*/( integer_TID: u16, integer_SID: u16 ) -> ();
//	//fn IDNo_random_calculate() -> ();
//}

/*#[cxx::bridge]
mod ffi
{
	unsafe extern "C++"
	{
		include!("./bin/rcg7tid/include/header.hpp");
		fn IDNo_random_calculate() -> ();
	}
}*/

//use cxx::*;
//use ffi::IDNo_random_calculate;
use std::env;
use std::io::*; //use std::io::{stdin,stdout,Write};
use std::str;
use std::string::String;
use libm::{log10,floor};
use rand::Rng;
use rand::thread_rng;
use num_traits::pow;
use std::fmt::Display;
use std::process;
use cidno::cg7tid::f_IDNo::f_IDNo; //use crate::cg7tid::f_IDNo::f_IDNo; //use crate::cg7tid::f_IDNo;
use cidno::bcg7tid::F_IDNo::F_IDNo; //use crate::bcg7tid::F_IDNo::F_IDNo; //use crate::bcg7tid::{l_n,F_IDNo};
use cidno::rcg7tid::R_IDNo::R_IDNo; //use crate::rcg7tid::R_IDNo::R_IDNo; //use crate::rcg7tid::R_IDNo;

fn use_help()
{
	print!("use: cidno [[[-]-c[alculate]] [n_TID] [n_SID]] [[[-]-b[ackward]|[-]-r[everse]] [n_G7TID]] [-R|--random[ly]]\n");
}

fn numeric_is(string: &str) -> bool
{
	string.chars().all( | c | c.is_numeric() )
}
/*fn numeric_is(string: String) -> bool
{
	let length: usize;
	let mut index: usize;

	length = string.chars().count();
	index = 0;
	while index < length
	{
		if string.as_str().chars().nth( index ).unwrap().is_numeric() == false
		{
			return false;
		}
	}
	return true;
}*/

fn main()
{
	let vector_argument: Vec<String> = env::args().collect();
	let count_argument: usize = vector_argument.len();
	let mut integer_TID: u16 = 0;
	let mut integer_SID: u16 = 0;
	let mut integer_TSV: i16 = 0;
	let mut integer_TRV: i8 = 0;
	let mut integer_G7SID: i16 = 0;
	let mut integer_G7TID: i32 = 0;
	let mut i_index: usize;
	let mut argument: &String;

	if count_argument == 1
	{
		( integer_TID, integer_SID, integer_TSV, integer_TRV, integer_G7SID, integer_G7TID ) = R_IDNo();
		print!("TID:\t{};\nSID:\t{};\nTSV:\t{};\nTRV:\t{};\nG7SID:\t{};\nG7TID:\t{};\n", integer_TID, integer_SID, integer_TSV, integer_TRV, integer_G7SID, integer_G7TID );
		std::process::exit( 0 );
	}
	i_index = 1;
	while i_index < count_argument
	{
		argument = &vector_argument[ i_index ];
		if argument == "--calculate" || argument == "-c" || ( i_index + 1 < count_argument && numeric_is( &vector_argument[ i_index ] ) && numeric_is( &vector_argument[ i_index + 1 ] ) )
		{
			if numeric_is( argument )
			{
				integer_TID = argument.parse::<u16>().unwrap();
				integer_SID = vector_argument[ i_index + 1 ].parse::<u16>().unwrap();
				i_index += 1;
			}
			else
			{
				integer_TID = vector_argument[ i_index + 1 ].parse::<u16>().unwrap();
				integer_SID = vector_argument[ i_index + 2 ].parse::<u16>().unwrap();
				i_index += 2;
			}
			( integer_TID, integer_SID, integer_TSV, integer_TRV, integer_G7TID, integer_G7SID ) = f_IDNo( integer_TID, integer_SID );
			print!("TID:\t{};\nSID:\t{};\nTSV:\t{};\nTRV:\t{};\nG7SID:\t{};\nG7TID:\t{};\n", integer_TID, integer_SID, integer_TSV, integer_TRV, integer_G7SID, integer_G7TID );
		}
		else if argument == "--backward" || argument == "--backwards" || argument == "-b" || argument == "--reverse" || argument == "-r"
		{
			if i_index + 1 < count_argument
			{
				integer_G7TID = vector_argument[ i_index + 1 ].parse::<i32>().unwrap();
				if integer_G7TID < 0 || integer_G7TID > 999999
				{
					eprint!("Error.\tInvalid target value.\nThe input value is not an integer number between 0 and 999999.\n");
					use_help();
					std::process::exit( 1 );
				}
				( integer_G7SID, integer_G7TID, integer_TRV, integer_TSV, integer_SID, integer_TID) = F_IDNo(integer_G7TID);
				print!("G7SID:\t{};\nG7TID:\t{};\nTRV:\t{};\nTSV:\t{};\nSID:\t{};\nTID:\t{};\n", integer_G7SID, integer_G7TID, integer_TRV, integer_TSV, integer_SID, integer_TID );
				i_index += 1;
			}
			else
			{
				eprint!("Error.\tMissing argument for backward calculation.\n");
				use_help();
				std::process::exit( 1 );
			}
		}
		else if argument == "--randomly" || argument == "--random" || argument == "-R"
		{
			( integer_TID, integer_SID, integer_TSV, integer_TRV, integer_G7SID, integer_G7TID ) = R_IDNo();
			print!("TID:\t{};\nSID:\t{};\nTSV:\t{};\nTRV:\t{};\nG7SID:\t{};\nG7TID:\t{};\n", integer_TID, integer_SID, integer_TSV, integer_TRV, integer_G7SID, integer_G7TID );
		}
		else if argument == "--help" || argument == "-h" || argument == "--use" || argument == "-u"
		{
			use_help();
			std::process::exit( 0 );
		}
		else
		{
			eprint!("Error.\tInvalid argument.\n");
			use_help();
			std::process::exit( 1 );
		}
		i_index += 1;
	}
}
/*fn main() // As usual read my routine, function and variable names backward, e.g. 'argument vector' for `vector_argument`.
{
	let vector_argument: Vec<String> = env::args().collect(); //dbg!( vector_argument );
	let count_argument: usize = vector_argument.len(); //dbg!( count_argument );
	let mut integer_TID: u16;
	let mut integer_SID: u16;
	let mut integer_TSV: i16;
	let mut integer_TRV: i8;
	let mut integer_G7SID: i16;
	let mut integer_G7TID: i32;
	let mut i_index: usize;

	if count_argument == 1
	{
		( integer_TID, integer_SID, integer_TSV, integer_TRV, integer_G7SID, integer_G7TID ) = R_IDNo();
		write!( stdout().lock(), "TID:\t{}\nSID:\t{}\nTSV:\t{}\nTRV:\t{}\nG7SID:\t{}\nG7TID:\t{}\n", integer_TID, integer_SID, integer_TSV, integer_TRV, integer_G7SID, integer_G7TID ).unwrap();
		//use_help();
		std::process::exit( 0 /*1*/ );
	}
	i_index = 0;
	while i_index <= count_argument
	{
		//match &vector_argument[ i_index ].as_str()
		//{
			if vector_argument[ i_index ].as_str() == "--calculate" || vector_argument[ i_index ].as_str() == "-c" || ( numeric_is( vector_argument[ i_index ].as_str().trim().to_string() ) && numeric_is( vector_argument[ i_index + 1 ].as_str().trim().to_string() ) ) //"--calculate" | "-c" | numeric_is( vector_argument[ i_index ].as_str().trim() ) /*&vector_argument[ i_index ].as_str().trim().parse::<u16>().is_ok()*/ =>
			{
				if vector_argument[ i_index ].as_str().trim().parse::<u16>().is_ok()
				{
					integer_TID = vector_argument[ i_index /*+ 0*/ ].parse::<u16>().unwrap();
					integer_SID = vector_argument[ i_index + 1 ].parse::<u16>().unwrap();
					/*unsafe
					{
						IDNo_calculate( integer_TID, integer_SID );
					}*/
					i_index += 1;
				}
				else
				{
					integer_TID = vector_argument[ i_index + 1 ].parse::<u16>().unwrap();
					integer_SID = vector_argument[ i_index + 2 ].parse::<u16>().unwrap();
					/*unsafe
					{
						IDNo_calculate( integer_TID, integer_SID );
					}*/
					i_index += 2;
				}
				//unsafe
				//{
					let ( integer_TID, integer_SID, integer_TSV, integer_TRV, integer_G7TID, integer_G7SID ) = f_IDNo( integer_TID, integer_SID ); //IDNo_calculate( integer_TID, integer_SID );
				//}
				/*if ( &vector_argument[ i_index ].as_str().trim().parse::<u16>().is_ok() )
				{
					i_index += 1;
				}
				else
				{
					i_index += 2;
				}*/
			}//,
			else if vector_argument[ i_index ].as_str() == "--backward" || vector_argument[ i_index ].as_str() == "-b" || vector_argument[ i_index ].as_str() == "--reverse" || vector_argument[ i_index ].as_str() == "-r" //"--backcalculate" | "-b" =>
			{
				integer_G7TID = vector_argument[ i_index + 1 ].parse::<i32>().unwrap();
				if integer_G7TID/*_target*/ < 0 || 999999 < integer_G7TID/*_target*/
				{
					eprintln!("Error: Invalid Target Value\nThe input value is not an integer number between 0 and 999999.");
					use_help();
					std::process::exit( 1 );
				}
				//integer_G7TID = &vector_argument[ i_index + 1 ].parse::<i32>().unwrap();
				( integer_G7SID, integer_G7TID, integer_TRV, integer_TSV, integer_SID, integer_TID ) = F_IDNo( integer_G7TID );
				write!( stdout().lock(), "G7SID:\t{}\nG7TID:\t{}\nTRV:\t{}\nTSV:\t{}\nSID:\t{}\nTID:\t{}\n", integer_G7SID, integer_G7TID, integer_TRV, integer_TSV, integer_SID, integer_TID ).unwrap();
				i_index += 1;
			}//,
			else if vector_argument[ i_index ].as_str() == "--randomly" || vector_argument[ i_index ].as_str() == "--random" || vector_argument[ i_index ].as_str() == "-R" /*|| count_argument == 0*/ //"--random" | "-r" =>
			{
				( integer_TID, integer_SID, integer_TSV, integer_TRV, integer_G7SID, integer_G7TID ) = R_IDNo();
				write!( stdout().lock(), "TID:\t{}\nSID:\t{}\nTSV:\t{}\nTRV:\t{}\nG7SID:\t{}\nG7TID:\t{}\n", integer_TID, integer_SID, integer_TSV, integer_TRV, integer_G7SID, integer_G7TID ).unwrap();
				/*unsafe
				{
					IDNo_random_calculate();
				}*/
				i_index += 1;
			}//,
			else if vector_argument[ i_index ].as_str() == "--help" || vector_argument[ i_index ].as_str() == "-h" || vector_argument[ i_index ].as_str() == "--use" || vector_argument[ i_index ].as_str() == "-u" //"--help" | "-h" =>
			{
				use_help();
			}//,
			else //_ =>
			{
				use_help();
				std::process::exit( 1 );
			}//,
		//}
		i_index += 1;
	}
	std::process::exit( 0 );
}*/

/*//	main.rs
////	CIDNo (IDNo Calculator)
////
////	Karl V. P. B. `kvpb`	AKA Karl Thomas George West `ktgw`
////	+33 A BB BB BB BB		+1 (DDD) DDD-DDDD
////	local-part@domain
////	https://twitter.com/ktgwkvpb
*///	https://github.com/kvpb

/*//	'[...]
////	Je hais ces vains auteurs, dont la muse est forcée
////	M'entretient de ses feux, toujours froide et glacée ;
////	Qui s'affligent par l'art, et, fous de sens rassis,
////	S'érigent pour rimer en amoureux transis.
////	Leurs transports les plus doux ne sont que phrases vaines.
////	Ils ne savent jamais que se charger de chaînes,
////	[...]
////	Loin ces rimeurs craintifs dont l'esprit flegmatique
////	Garde dans ses fureurs un ordre didactique,
////	Qui, chantant d'un héros les progrès éclatants,
////	Maigres historiens, suivront l'ordre des temps !
////	[...]
////	Apollon de son feu leur fut toujours avare.
////	On dit, à ce propos, qu'un jour ce dieu bizarre,
////	Voulant pousser à bout tous les rimeurs françois,
////	Inventat du Sonnet les rigoureuses lois ;
////	[...]
////	Du reste, il l'enrichit d'une beauté suprême
////	Un sonnet sans défaut vaut seul un long Poème.
////	Mais en vain mille auteurs y pensent arriver,
////	Et cet heureux phénix est encore à trouver.
////	À peine dans Gombaut, Maynard et Malleville,
////	En peut-on admirer deux ou trois entre mille ;
////	Le reste, aussi peu lu que ceux de Pelletier,
////	N'a fait de chez Sercy, qu'un saut chez l'épicier.
////	[...]
////	L'Épigramme, plus libre en son tour plus borné,
////	N'est souvent qu'un bon mot de deux rimes orné.
////	[...]
////	La raison outragée enfin ouvrit les yeux,
////	La chassa pour jamais des discours sérieux ;
////	Et, dans tous ces écrits la déclarant infâme,
////	Par grâce lui laissa l'entrée en l'épigramme,
////	Pourvu que sa finesse, éclatant à propos,
////	Roulât sur la pensée et non pas sur les mots.
////	[...]
////	Ce n'est pas quelquefois qu'une Muse un peu fine,
////	Sur un mot, en passant, ne joue et ne badine,
////	Et d'un sens détourné n'abuse avec succès
////	Mais fuyez sur ce point un ridicule excès,
////	Et n'allez pas toujours d'une pointe, frivole
////	Aiguiser par la queue une épigramme folle.
////	Tout poème est brillant de sa propre beauté.
////	Le Rondeau, né gaulois, a la naïveté.
////	La Ballade, asservie à ses vieilles maximes,
////	Souvent doit tout son lustre au caprice des rimes.
////	Le Madrigal, plus simple et plus noble en son tour,
////	Respire la douceur, la tendresse et l'amour.
////	L'ardeur de se montrer, et non pas de médire,
////	Arma la Vérité du vers de la Satire.
////	Lucile le premier osa la faire voir,
////	Aux vices des Romains présenta le miroir,
////	Vengea l'humble vertu de la richesse altière,
////	Et l'honnête homme à pied du faquin en litière.
////	Horace à cette aigreur mêla son enjouement
////	On ne fut plus ni fat ni sot impunément ;
////	Et malheur à tout nom qui, propre à la censure
////	Put entrer dans un vers sans rompre la mesure !
////	Perse, en ses vers obscurs, mais serrés et pressants,
////	Affecta d'enfermer moins de mots que de sens.
////	Juvénal, élevé dans les cris de l'école,
////	Poussa jusqu'à l'excès sa mordante hyperbole.
////	Ses ouvrages, tout pleins d'affreuses vérités,
////	Étincellants pourtant de sublimes beautés
////	Soit que, sur un écrit arrivé de Caprée,
////	Il brise de Séjan la statue adorée ;
////	Soit qu'il fasse au conseil courir les sénateurs,
////	D'un tyran soupçonneux pâles adulateurs ;
////	Ou que, poussant à bout la luxure latine,
////	Aux portefaix de Rome il vende Messaline,
////	Ses écrits pleins de feu partout brillent aux yeux.
////	De ces maîtres savants disciple ingénieux,
////	Régnier seul parmi nous formé sur leurs modèles,
////	Dans son vieux style encore a des grâces nouvelles.
////	Heureux, si ses discours, craints du chaste lecteur,
////	Ne se sentaient des lieux où fréquentait l'auteur,
////	Et si, du son hardi de ses rimes cyniques,
////	Il n'alarmait souvent les oreilles pudiques !
////	[...]
////	Je veux dans la satire un esprit de candeur,
////	Et fuis un effronté qui prêche la pudeur.
////	D'un trait de ce poème en bons mots si fertile,
////	Le Français, né malin, forma le Vaudeville,
////	Agréable indiscret qui, conduit par le chant,
////	Passe de bouche en bouche et s'accroît en marchant.
////	La liberté française en ses vers se déploie :
////	Cet enfant du plaisir veut naître dans la joie.
////	Toutefois n'allez pas, goguenard dangereux,
////	Faire Dieu le sujet d'un badinage affreux.
////	À la fin tous ces jeux, que l'athéisme élève,
////	Conduisent tristement le plaisant à la Grève.
////	Il faut, même en chansons, du bon sens et de l'art
////	Mais pourtant on a vu le vin et le hasard
////	Inspirer quelquefois une Muse grossière
////	Et fournir, sans génie, un couplet à Linière.
////	Mais, pour un vain bonheur qui vous a fait rimer,
////	Gardez qu'un sot orgueil ne vous vienne enfumer.
////	Souvent, l'auteur altier de quelque chansonnette
////	Au même instant prend droit de se croire poète
////	Il ne dormira plus qu'il n'ait fait un sonnet,
////	Il met tous les matins six impromptus au net.
////	Encore est-ce un miracle, en ses vagues furies,
////	Si bientôt, imprimant ses sottes rêveries,
////	Il ne se fait graver au-devant du recueil,
*///	Couronné de lauriers, par la main de Nanteuil.'
