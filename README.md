<!-- Let the good times roll! -->

<p><b>Caution.</b> Using this software might make your penis grow by a few inches, or your breasts by a few cups. Use it at your own risk.<br>
<b>Attention.</b> Utiliser ce logiciel risquerait de faire croître votre pénis de quelques doubles centimètres et quart, ou votre poitrine de quelques bonnets. Servez-vous-en à vos risques et périls.<br></p>

<p align="center"><img src="https://media1.tenor.com/m/fgAa5IuzjPsAAAAd/casino-dice-spin.gif"><br>
<i>What's your number?</i> &#x1F4AF;</p>

<h3 align="center"><b>T.G.'s</b></h3>
<h1 align="center"><b>CIDNo (IDNo<!-- (TID, SID, TSV, TRV, G7SID & G7TID)--> Calculator)</b></h1>
<h3 align="center"><b>A generation I, III & VII secret & trainer ID and trainer shiny & residual value calculator.</b><br>
<b>Un calculateur de numéros d'ID & secret de générations I, III & VII et valeurs chromatique & résiduelle.</b></h3> <!-- No shit. -->

![version](https://img.shields.io/badge/version-4.20-yellow) ![Rust](https://img.shields.io/badge/-Rust-F46623?style=flat&logo=rust&logoColor=FFF) <!--![C++](https://img.shields.io/badge/-C++-649AD2?style=flat&logo=cplusplus&logoColor=004482)--> <!--![C](https://img.shields.io/badge/-C-1B75B3?style=flat&logo=c&logoColor=A8B9CC)--> <!--![Makefile](https://img.shields.io/badge/-Makefile-848484?style=flat&logo=cmake&logoColor=D2D2D2)--> ![macOS](https://img.shields.io/badge/-macOS-FFF?style=flat&logo=apple&logoColor=A7A9AC) ![Linux](https://img.shields.io/badge/-Linux-0094FF?style=flat&logo=linux&logoColor=003778) ![platform](https://gistcdn.githack.com/kvpb/ed57eb3a03f2b5338fbede97e7cf296b/raw/ea6556bb04f592433023533ddbd931cc7d23e92b/apple-F6F6F6D5E1ED1E72F21AD5FD-madeona-1AD5FD1E72F2-mac-F6F6F6D5E1ED.svg)

<pre align="center">
cidno (((-)-c(alculate))               {n_TID} {n_SID})
      (((-)-b(ackward(s))||(-)-r(everse)) {n_G7TID})
          (-R||--random(ly))
       ((-)-h(elp)||(-)-u(se))
</pre>

---

# &#x1F1FA;&#x1F1F8; US English

This software is an UNIX command-line utility currently programmed in [Rust](https://www.rust-lang.org/)<!--, [C++](https://isocpp.org/)--><!-- and [C](https://www.open-std.org/jtc1/sc22/wg14/)-->. It as such requires an [UNIX](http://opengroup.org/unix)-certified or [POSIX](http://get.posixcertified.ieee.org/)-compliant operating system or operating environment such as [macOS](https://www.apple.com/macos) with [Command Line Tools](https://developer.apple.com/library/archive/technotes/tn2339/_index.html), [FreeBSD](https://www.freebsd.org/), [Arch Linux](https://archlinux.org/), [Fedora Linux](https://fedoraproject.org/), a standard command-line shell like [BASH](https://www.gnu.org/software/bash/), [ZSH](https://www.zsh.org/) or [TCSH](https://www.tcsh.org/) and<!--,--> the official [Rust](https://www.rust-lang.org/) programming language toolchain<!--, the [GNU Compiler Collection](https://gcc.gnu.org/) or [CLANG](https://clang.llvm.org/) standard C--><!-- and C++--><!-- optimizing compilers--> and the [Make](https://www.gnu.org/software/make/) build automation tool. The user may set it up and use it from a command-line interface in a graphical user interface via a terminal emulator, e.g. [iTerm2](https://iterm2.com), [Terminator](https://gnome-terminator.org) or [the integrated terminal of Visual Studio Code](https://code.visualstudio.com/docs/editor/integrated-terminal). These instructions assume the user knows how to use an UNIX shell<!--, e.g. [BASH](https://www.gnu.org/software/bash/), [ZSH](https://www.zsh.org/) or [TCSH](https://www.tcsh.org/),--> in a POSIX-compliant development and runtime environment<!--, e.g. [macOS](https://www.apple.com/macos/) or [a Linux-based operating system](https://www.linux.org/pages/download/)-->.<br>
&nbsp;&nbsp;&nbsp;&nbsp;The user shall note 'generation N ID number' in this manual refers to all types of numbers from generation I to generation N, e.g. a generation I trainer ID number, a generation III secret ID number, a generation IV trainer shiny value, a generation IV trainer residual value, a generation VII trainer ID number and a generation VII secret ID number by 'generation VII ID number'. The user also may have noticed 'IDNo' originally meant the generation VII trainer ID number in the Japanese versions of the games but means in this source code the full generation VII ID number not split into a generation VII trainer ID number and a generation VII secret ID number.

## Set up

Let us now declare a few functions and variables for the sake of brevity. The user of course can forgo this and do it all by themselves manually, **but if you do so, please do not ask to be helped with this software.**

```sh
pathname()
{
	printf "$(cd "$(dirname "${1}")" && pwd -P)/$(basename "${1}")"'\n'
}

URL="https://github.com/kvpb/idnoc"
dir="${URL%/*}" && dir="${dir##*/}"
repo="${URL##*/}"
progname="${repo}"
bin="$(pathname ${repo})/${progname}"
R[0]=$(( ( 1 + $[RANDOM%2] ) * $[RANDOM] )) # n_TID
R[1]=$(( ( 1 + $[RANDOM%2] ) * $[RANDOM] )) # n_SID
R[255]=$(( ${R[0]} + ( 2**16 * ${R[1]} ) )) # n_G7ID
R[2]=$(( ${R[255]} % 10**6 )) # n_G7TID
```

**The user should avoid setting this software up from a critical location in the file system such as `/` or the parent directory of other software.** I advise doing so from a dedicated, throwaway directory. The user therefore should do thus:

```sh
mkdir ${dir} # preferably from ${HOME}.
cd ${dir}
```

Once the user has obtained the sources of this software, they may proceed to compile them. They should do thus:

```sh
git clone ${URL}
cd ${repo}
# preferably from ${dir}/.
make # from ${repo}/.
```

The executable file at this point should be in `${repo}/bin/`. The user should be able to execute it.

## Use

The user at this point may use the software. Simply enter the pathname of the executable file, the option of the chosen mode and the necessary arguments. They can do thus:

```sh
cd bin # from ${repo}/.
```

If the user wants to calculate a generation VII ID number, they shall do so with or without the (`-`)`-c`(`alculate`) option and `i_TID` and `i_SID` arguments:

```bash
./${bin} --calculate ${R[0]} ${R[1]}
./${bin} -c ${R[0]} ${R[1]}
./${bin} ${R[0]} ${R[1]}
# from any directory in the file system.
```

If the user wants to backward calculate a generation VII ID number from a generation VII trainer ID number, they must do so with the (`-`)`-b`(`ackward`) or (`-`)`-r`(`everse`) options and an `n_G7TID` argument:

```zsh
./${bin} --reverse ${R[2]}
./${bin} -r ${R[2]}
./${bin} --backward ${R[2]}
./${bin} -b ${R[2]}
# from any directory in the file system.
```

If the user wants to randomly generate a generation VII ID number, they have to do so with or without the `-R` (`--random`(`ly`)) option but no argument:

```tcsh
./${bin} --randomly
./${bin} --random
./${bin} -R
./${bin}
# from any directory in the file system.
```

If the user desires to know how to use this software, they need to do so with the (`-`)`-h`(`elp`) or (`-`)`-u`(`se`) option and no argument or wrong ones:

```fish
./${bin} --help
./${bin} -h
./${bin} --use
./${bin} -u
./${bin} 0
./${bin} F 0
./${bin} -b 0 0
./${bin} -R 0
# from any directory in the file system.
```

# &#x1F1EB;&#x1F1F7; Français FR

Ce logiciel est un utilitaire en ligne de commande pour le moment programmé en [Rust](https://www.rust-lang.org/)<!--, [C++](https://isocpp.org/) et [C](https://www.open-std.org/jtc1/sc22/wg14/)-->. Il requiert un système d'exploitation ou environnement d'exploitation certifié [UNIX](http://opengroup.org/unix) ou conforme à [POSIX](http://get.posixcertified.ieee.org/) tel que [macOS](https://www.apple.com/macos) avec [Command Line Tools](https://developer.apple.com/library/archive/technotes/tn2339/_index.html), [FreeBSD](https://www.freebsd.org/), [Arch Linux](https://archlinux.org/), [CRUX](https://crux.nu/) ou [Alpine Linux](https://alpinelinux.org/), un interpréteur de commandes standard comme [BASH](https://www.gnu.org/software/bash/), [ZSH](https://www.zsh.org/) ou [TCSH](https://www.tcsh.org/), la chaîne de compilation officielle du langage de programmation [Rust](https://www.rust-lang.org/)<!--, un compilateur d'optimisation standard du--><!--es--><!-- C--><!-- et C++--><!-- [GNU Compiler Collection](https://gcc.gnu.org/) ou [CLANG](https://clang.llvm.org/)--> et le moteur de production [Make](https://www.gnu.org/software/make/). L'utilisateur peut l'installer et utiliser depuis une interface en ligne de commande dans une interface graphique via un terminal virtuel, e.g. [iTerm2](https://iterm2.com), [Terminator](https://gnome-terminator.org) ou [le terminal intégré de Visual Studio Code](https://code.visualstudio.com/docs/editor/integrated-terminal). Ces instructions supposent l'utilisateur savoir se servir d'un interpréteur de commandes pour UNIX dans un environnement de développement et d'exécution conforme à POSIX. <!-- Holy shit. Never had I translated English computer instructions in French before. I never even had read computing terminology in French. I just read it in English for the first time of my life. I always set my Windows PC in English, my PlayStation 3 in English, my iPhone 3GS in English, my MacBook Air in English, my Xbox 360 in English... That felt weird. I became another stranger in another homeland for a few minutes. Shit, man. What am I? English, Irish, Scottish? German? French? Spanish? I genetically come from each of these, but of which am I the most at least? Am I all of them? Or am I just the West embodied? I am, am I not? -->
<br>&nbsp;&nbsp;&nbsp;&nbsp;L'utilisateur doit comprendre 'numéro d'ID de génération N' dans ce manuel sous-entendre tous les types de numéros de la génération I à la génération N, e.g. un numéro d'ID de génération I, un numéro secret de génération III, une valeur de chromatique de génération IV, une valeur résiduelle de chromatique de génération IV, un numéro d'ID de génération VII et un numéro secret de génération VII par 'numéro de génération VII'. L'utilisateur aussi a pu remarquer 'IDNo' initialement signifier dans les versions japonaises le numéro d'ID de génération VII mais vouloir dire dans le source code de ce logiciel le numéro de génération VII entier pas encore scindé en un numéro d'ID de génération VII et un numéro secret de génération VII.

## Installer

Déclarons à présent quelques fonctions et variables par souci de brièveté. L'utilisateur peut bien sûr passer outre et faire tout ceci par lui-même, **mais si vous le faites, veuillez en cas de problème ne pas demander à être aidé avec ce logiciel.**

```sh
pathname()
{
	printf "$(cd "$(dirname "${1}")" && pwd -P)/$(basename "${1}")"'\n'
}

URL="https://github.com/kvpb/idnoc"
dir="${URL%/*}" && dir="${dir##*/}"
repo="${URL##*/}"
progname="${repo}"
bin="$(pathname ${repo})/${progname}"
R[0]=$(( ( 1 + $[RANDOM%2] ) * $[RANDOM] )) # R_TID
R[1]=$(( ( 1 + $[RANDOM%2] ) * $[RANDOM] )) # R_SID
R[255]=$(( ${R[0]} + ( 2**16 * ${R[1]} ) )) # R_G7ID
R[2]=$(( ${R[255]} % 10**6 )) # R_G7TID
```

**L'utilisateur devrait éviter d'installer ce logiciel depuis un point critique du système de fichiers tel que `/` ou le répertoire parent d'un autre logiciel.** Je conseille de le faire depuis un répertoire temporaire dédié. L'utilisateur peut alors procéder ainsi :

```sh
mkdir ${dir} # de preference depuis ${HOME}.
cd ${dir}
```

Lorsque l'utilisateur a obtenu les sources de ce logiciel, il peut en venir à les compiler. Il devrait procéder ainsi :

```sh
git clone ${URL}
cd ${repo}
# de preference depuis ${dir}/.
make # depuis ${repo}/.
```

Le fichier exécutable est censé à ce stade se trouver dans `${repo}/bin/`. L'utilisateur devrait pouvoir le faire s'exécuter.

## Utiliser

L'utilisateur peut désormais utiliser le logiciel. Entrez seulement le chemin d'accès du fichier exécutable, l'option du mode choisi et les arguments nécessaires. Il peut procéder ainsi :

```sh
cd bin # depuis ${repo}/.
```

Si l'utilisateur veut calculer un numéro de génération VII, il doit y procéder avec ou sans l'option (`-`)`-c`(`alculate`) et des arguments `i_TID` et `i_SID`:

```bash
./${bin} --calculate ${R[0]} ${R[1]}
./${bin} -c ${R[0]} ${R[1]}
./${bin} ${R[0]} ${R[1]}
# depuis tout repertoire dans le systeme de fichiers.
```

Si l'utilisateur veut inversement calculer un numéro de génération VII à partir d'un numéro d'ID de génération VII, il lui faut y procéder avec les options (`-`)`-b`(`ackward`) ou (`-`)`-r`(`everse`) et un argument `n_G7TID`:

```zsh
./${bin} --reverse ${R[2]}
./${bin} -r ${R[2]}
./${bin} --backward ${R[2]}
./${bin} -b ${R[2]}
# depuis tout repertoire dans le systeme de fichiers.
```

Si l'utilisateur veut générer au hasard un numéro de génération VII, il a à y procéder avec ou sans l'option `-R` (`--random`(`ly`)) mais sans argument:

```tcsh
./${bin} --randomly
./${bin} --random
./${bin} -R
./${bin}
# depuis tout repertoire dans le systeme de fichiers.
```

Si l'utilisateur désire savoir comment se servir du logiciel, il lui requiert d'y procéder avec les options (`-`)`-h`(`elp`) ou (`-`)`-u`(`se`) et pas d'argument ou de mauvais:

```fish
./${bin} --help
./${bin} -h
./${bin} --use
./${bin} -u
./${bin} 0
./${bin} F 0
./${bin} -b 0 0
./${bin} -R 0
# depuis tout repertoire dans le systeme de fichiers.
```

<!-- I am. I am a native full west european. And I will become a full north american on top of that. But I gotta get to be a doctor first. -->

---

<p align="center"><br>
<a href="http://kvpb.co"><img src="https://gistcdn.githack.com/kvpb/c80594e9079e857c55c36dec49a1a2d7/raw/eec54d5821dc092ad910635141c4e4feebf07565/kvpbssymbol.svg"></a><br>
<b>Karl V. P. B. `<code>kvpb</code>`<br>AKA&nbsp;&nbsp;Karl <!--&lsquo;T.G.&rsquo;--> Thomas George <!--<span style="font-variant: small-caps;">-->West<!--</span>--> `<code>ktgw</code>`</b><br>
software engineer<br>
<br>
<span style="display:block;text-align:center"><a href="https://www.linkedin.com/in/karlbertin"><img src="https://gistcdn.githack.com/kvpb/6934ca4100436428368c4d2418633755/raw/2faac08dba4920c280aa337bac2f40c1fb991673/linkedin.svg" alt="LinkedIn"></a>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<a href="https://twitter.com/ktgwkvpb"><img src="https://gistcdn.githack.com/kvpb/20db04ea32721c7a968f198dbbdf688d/raw/1b2126ef7f572709f8cca49216e0021f324a2639/twitter.svg" alt="Twitter"></a>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<a href="https://vm.tiktok.com/ZSwAmcFh/"><img src="https://gistcdn.githack.com/kvpb/bc289a1af9975c9603159dbdf77778d3/raw/6ffe1058cd7e87b0c6239352b01691615a4f74c2/tiktok.svg" alt="TikTok"></a></span><br>
<img src="https://gistcdn.githack.com/kvpb/f5f75716dd024cc48f8dc28176c0b642/raw/e5344bebf926b378faeb208724ae97f4e06639ca/kvpbsesrbrating.svg"></p>
