const fs = require("fs")
let arr = [
  {
    "link": "https:learnxinyminutes.com/docs/asymptotic-notation/",
    "name": "Asymptotic Notation"
  },
  {
    "link": "https:learnxinyminutes.com/docs/dynamic-programming/",
    "name": "Dynamic Programming"
  },
  {
    "link": "https:learnxinyminutes.com/docs/lambda-calculus/",
    "name": "Lambda Calculus"
  },
  {
    "link": "https:learnxinyminutes.com/docs/set-theory/",
    "name": "Set theory"
  },
  {
    "link": "https:learnxinyminutes.com/docs/asciidoc/",
    "name": "asciidoc"
  },
  {
    "link": "https:learnxinyminutes.com/docs/bc/",
    "name": "bc"
  },
  {
    "link": "https:learnxinyminutes.com/docs/bf/",
    "name": "bf"
  },
  {
    "link": "https:learnxinyminutes.com/docs/c/",
    "name": "C"
  },
  {
    "link": "https:learnxinyminutes.com/docs/csharp/",
    "name": "C#"
  },
  {
    "link": "https:learnxinyminutes.com/docs/c++/",
    "name": "C++"
  },
  {
    "link": "https:learnxinyminutes.com/docs/cairo/",
    "name": "Cairo"
  },
  {
    "link": "https:learnxinyminutes.com/docs/chapel/",
    "name": "chapel"
  },
  {
    "link": "https:learnxinyminutes.com/docs/CHICKEN/",
    "name": "CHICKEN"
  },
  {
    "link": "https:learnxinyminutes.com/docs/clojure/",
    "name": "clojure"
  },
  {
    "link": "https:learnxinyminutes.com/docs/clojure-macros/",
    "name": "clojure macros"
  },
  {
    "link": "https:learnxinyminutes.com/docs/cobol/",
    "name": "COBOL"
  },
  {
    "link": "https:learnxinyminutes.com/docs/coffeescript/",
    "name": "coffeescript"
  },
  {
    "link": "https:learnxinyminutes.com/docs/coldfusion/",
    "name": "coldfusion"
  },
  {
    "link": "https:learnxinyminutes.com/docs/common-lisp/",
    "name": "Common Lisp"
  },
  {
    "link": "https:learnxinyminutes.com/docs/coq/",
    "name": "Coq"
  },
  {
    "link": "https:learnxinyminutes.com/docs/crystal/",
    "name": "crystal"
  },
  {
    "link": "https:learnxinyminutes.com/docs/css/",
    "name": "css"
  },
  {
    "link": "https:learnxinyminutes.com/docs/cypher/",
    "name": "cypher"
  },
  {
    "link": "https:learnxinyminutes.com/docs/d/",
    "name": "D"
  },
  {
    "link": "https:learnxinyminutes.com/docs/dart/",
    "name": "dart"
  },
  {
    "link": "https:learnxinyminutes.com/docs/dhall/",
    "name": "Dhall"
  },
  {
    "link": "https:learnxinyminutes.com/docs/easylang/",
    "name": "Easylang"
  },
  {
    "link": "https:learnxinyminutes.com/docs/edn/",
    "name": "edn"
  },
  {
    "link": "https:learnxinyminutes.com/docs/elisp/",
    "name": "elisp"
  },
  {
    "link": "https:learnxinyminutes.com/docs/elixir/",
    "name": "elixir"
  },
  {
    "link": "https:learnxinyminutes.com/docs/elm/",
    "name": "Elm"
  },
  {
    "link": "https:learnxinyminutes.com/docs/erlang/",
    "name": "erlang"
  },
  {
    "link": "https:learnxinyminutes.com/docs/fsharp/",
    "name": "F#"
  },
  {
    "link": "https:learnxinyminutes.com/docs/factor/",
    "name": "factor"
  },
  {
    "link": "https:learnxinyminutes.com/docs/forth/",
    "name": "forth"
  },
  {
    "link": "https:learnxinyminutes.com/docs/fortran90/",
    "name": "Fortran"
  },
  {
    "link": "https:learnxinyminutes.com/docs/func/",
    "name": "FunC"
  },
  {
    "link": "https:learnxinyminutes.com/docs/gdscript/",
    "name": "GDScript"
  },
  {
    "link": "https:learnxinyminutes.com/docs/go/",
    "name": "Go"
  },
  {
    "link": "https:learnxinyminutes.com/docs/groovy/",
    "name": "Groovy"
  },
  {
    "link": "https:learnxinyminutes.com/docs/hack/",
    "name": "Hack"
  },
  {
    "link": "https:learnxinyminutes.com/docs/haml/",
    "name": "haml"
  },
  {
    "link": "https:learnxinyminutes.com/docs/haskell/",
    "name": "Haskell"
  },
  {
    "link": "https:learnxinyminutes.com/docs/haxe/",
    "name": "haxe"
  },
  {
    "link": "https:learnxinyminutes.com/docs/hdl/",
    "name": "hdl"
  },
  {
    "link": "https:learnxinyminutes.com/docs/hjson/",
    "name": "Hjson"
  },
  {
    "link": "https:learnxinyminutes.com/docs/hq9+/",
    "name": "HQ9+"
  },
  {
    "link": "https:learnxinyminutes.com/docs/html/",
    "name": "html"
  },
  {
    "link": "https:learnxinyminutes.com/docs/hy/",
    "name": "hy"
  },
  {
    "link": "https:learnxinyminutes.com/docs/inform7/",
    "name": "Inform7"
  },
  {
    "link": "https:learnxinyminutes.com/docs/janet/",
    "name": "Janet"
  },
  {
    "link": "https:learnxinyminutes.com/docs/java/",
    "name": "java"
  },
  {
    "link": "https:learnxinyminutes.com/docs/javascript/",
    "name": "javascript"
  },
  {
    "link": "https:learnxinyminutes.com/docs/json/",
    "name": "json"
  },
  {
    "link": "https:learnxinyminutes.com/docs/jsonnet/",
    "name": "jsonnet"
  },
  {
    "link": "https:learnxinyminutes.com/docs/julia/",
    "name": "Julia"
  },
  {
    "link": "https:learnxinyminutes.com/docs/kdb+/",
    "name": "kdb+"
  },
  {
    "link": "https:learnxinyminutes.com/docs/kotlin/",
    "name": "kotlin"
  },
  {
    "link": "https:learnxinyminutes.com/docs/latex/",
    "name": "latex"
  },
  {
    "link": "https:learnxinyminutes.com/docs/lbstanza/",
    "name": "LB Stanza"
  },
  {
    "link": "https:learnxinyminutes.com/docs/ldpl/",
    "name": "LDPL"
  },
  {
    "link": "https:learnxinyminutes.com/docs/less/",
    "name": "less"
  },
  {
    "link": "https:learnxinyminutes.com/docs/lfe/",
    "name": "Lisp Flavoured Erlang(LFE)"
  },
  {
    "link": "https:learnxinyminutes.com/docs/livescript/",
    "name": "LiveScript"
  },
  {
    "link": "https:learnxinyminutes.com/docs/logtalk/",
    "name": "Logtalk"
  },
  {
    "link": "https:learnxinyminutes.com/docs/LOLCODE/",
    "name": "LOLCODE"
  },
  {
    "link": "https:learnxinyminutes.com/docs/lua/",
    "name": "Lua"
  },
  {
    "link": "https:learnxinyminutes.com/docs/m/",
    "name": "M (MUMPS)"
  },
  {
    "link": "https:learnxinyminutes.com/docs/markdown/",
    "name": "markdown"
  },
  {
    "link": "https:learnxinyminutes.com/docs/matlab/",
    "name": "Matlab"
  },
  {
    "link": "https:learnxinyminutes.com/docs/mercury/",
    "name": "mercury"
  },
  {
    "link": "https:learnxinyminutes.com/docs/mips/",
    "name": "MIPS Assembly"
  },
  {
    "link": "https:learnxinyminutes.com/docs/mongodb/",
    "name": "MongoDB"
  },
  {
    "link": "https:learnxinyminutes.com/docs/montilang/",
    "name": "montilang"
  },
  {
    "link": "https:learnxinyminutes.com/docs/moonscript/",
    "name": "moonscript"
  },
  {
    "link": "https:learnxinyminutes.com/docs/neat/",
    "name": "neat"
  },
  {
    "link": "https:learnxinyminutes.com/docs/nim/",
    "name": "Nim"
  },
  {
    "link": "https:learnxinyminutes.com/docs/nix/",
    "name": "nix"
  },
  {
    "link": "https:learnxinyminutes.com/docs/objective-c/",
    "name": "Objective-C"
  },
  {
    "link": "https:learnxinyminutes.com/docs/ocaml/",
    "name": "OCaml"
  },
  {
    "link": "https:learnxinyminutes.com/docs/openscad/",
    "name": "openscad"
  },
  {
    "link": "https:learnxinyminutes.com/docs/paren/",
    "name": "Paren"
  },
  {
    "link": "https:learnxinyminutes.com/docs/pascal/",
    "name": "Pascal"
  },
  {
    "link": "https:learnxinyminutes.com/docs/pcre/",
    "name": "PCRE"
  },
  {
    "link": "https:learnxinyminutes.com/docs/perl/",
    "name": "perl"
  },
  {
    "link": "https:learnxinyminutes.com/docs/phel/",
    "name": "phel"
  },
  {
    "link": "https:learnxinyminutes.com/docs/php/",
    "name": "PHP"
  },
  {
    "link": "https:learnxinyminutes.com/docs/raku-pod/",
    "name": "Pod"
  },
  {
    "link": "https:learnxinyminutes.com/docs/pogo/",
    "name": "pogoscript"
  },
  {
    "link": "https:learnxinyminutes.com/docs/processing/",
    "name": "processing"
  },
  {
    "link": "https:learnxinyminutes.com/docs/prolog/",
    "name": "prolog"
  },
  {
    "link": "https:learnxinyminutes.com/docs/protocol-buffer-3/",
    "name": "protocol-buffers"
  },
  {
    "link": "https:learnxinyminutes.com/docs/pug/",
    "name": "Pug"
  },
  {
    "link": "https:learnxinyminutes.com/docs/purescript/",
    "name": "PureScript"
  },
  {
    "link": "https:learnxinyminutes.com/docs/python/",
    "name": "Python"
  },
  {
    "link": "https:learnxinyminutes.com/docs/pythonlegacy/",
    "name": "Python 2 (legacy)"
  },
  {
    "link": "https:learnxinyminutes.com/docs/qsharp/",
    "name": "Q#"
  },
  {
    "link": "https:learnxinyminutes.com/docs/r/",
    "name": "R"
  },
  {
    "link": "https:learnxinyminutes.com/docs/racket/",
    "name": "racket"
  },
  {
    "link": "https:learnxinyminutes.com/docs/raku/",
    "name": "Raku"
  },
  {
    "link": "https:learnxinyminutes.com/docs/rdf/",
    "name": "RDF"
  },
  {
    "link": "https:learnxinyminutes.com/docs/reason/",
    "name": "reason"
  },
  {
    "link": "https:learnxinyminutes.com/docs/red/",
    "name": "Red"
  },
  {
    "link": "https:learnxinyminutes.com/docs/rst/",
    "name": "restructured text (RST)"
  },
  {
    "link": "https:learnxinyminutes.com/docs/ruby/",
    "name": "ruby"
  },
  {
    "link": "https:learnxinyminutes.com/docs/rust/",
    "name": "Rust"
  },
  {
    "link": "https:learnxinyminutes.com/docs/sass/",
    "name": "sass"
  },
  {
    "link": "https:learnxinyminutes.com/docs/scala/",
    "name": "Scala"
  },
  {
    "link": "https:learnxinyminutes.com/docs/self/",
    "name": "self"
  },
  {
    "link": "https:learnxinyminutes.com/docs/sing/",
    "name": "Sing"
  },
  {
    "link": "https:learnxinyminutes.com/docs/smallbasic/",
    "name": "SmallBASIC"
  },
  {
    "link": "https:learnxinyminutes.com/docs/smalltalk/",
    "name": "Smalltalk"
  },
  {
    "link": "https:learnxinyminutes.com/docs/solidity/",
    "name": "Solidity"
  },
  {
    "link": "https:learnxinyminutes.com/docs/sql/",
    "name": "SQL"
  },
  {
    "link": "https:learnxinyminutes.com/docs/standard-ml/",
    "name": "Standard ML"
  },
  {
    "link": "https:learnxinyminutes.com/docs/stylus/",
    "name": "stylus"
  },
  {
    "link": "https:learnxinyminutes.com/docs/swift/",
    "name": "swift"
  },
  {
    "link": "https:learnxinyminutes.com/docs/tcl/",
    "name": "Tcl"
  },
  {
    "link": "https:learnxinyminutes.com/docs/texinfo/",
    "name": "Texinfo"
  },
  {
    "link": "https:learnxinyminutes.com/docs/textile/",
    "name": "textile"
  },
  {
    "link": "https:learnxinyminutes.com/docs/toml/",
    "name": "toml"
  },
  {
    "link": "https:learnxinyminutes.com/docs/typescript/",
    "name": "TypeScript"
  },
  {
    "link": "https:learnxinyminutes.com/docs/uxntal/",
    "name": "uxntal"
  },
  {
    "link": "https:learnxinyminutes.com/docs/vala/",
    "name": "vala"
  },
  {
    "link": "https:learnxinyminutes.com/docs/vimscript/",
    "name": "Vimscript"
  },
  {
    "link": "https:learnxinyminutes.com/docs/visualbasic/",
    "name": "Visual Basic"
  },
  {
    "link": "https:learnxinyminutes.com/docs/vyper/",
    "name": "Vyper"
  },
  {
    "link": "https:learnxinyminutes.com/docs/wasm/",
    "name": "WebAssembly"
  },
  {
    "link": "https:learnxinyminutes.com/docs/whip/",
    "name": "whip"
  },
  {
    "link": "https:learnxinyminutes.com/docs/wolfram/",
    "name": "wolfram"
  },
  {
    "link": "https:learnxinyminutes.com/docs/xml/",
    "name": "xml"
  },
  {
    "link": "https:learnxinyminutes.com/docs/yaml/",
    "name": "yaml"
  },
  {
    "link": "https:learnxinyminutes.com/docs/zig/",
    "name": "zig"
  },
  {
    "link": "https:learnxinyminutes.com/docs/amd/",
    "name": "amd"
  },
  {
    "link": "https:learnxinyminutes.com/docs/angularjs/",
    "name": "AngularJS"
  },
  {
    "link": "https:learnxinyminutes.com/docs/ansible/",
    "name": "ansible"
  },
  {
    "link": "https:learnxinyminutes.com/docs/awk/",
    "name": "awk"
  },
  {
    "link": "https:learnxinyminutes.com/docs/bash/",
    "name": "bash"
  },
  {
    "link": "https:learnxinyminutes.com/docs/cmake/",
    "name": "cmake"
  },
  {
    "link": "https:learnxinyminutes.com/docs/compojure/",
    "name": "compojure"
  },
  {
    "link": "https:learnxinyminutes.com/docs/php-composer/",
    "name": "composer"
  },
  {
    "link": "https:learnxinyminutes.com/docs/directx9/",
    "name": "DirectX 9"
  },
  {
    "link": "https:learnxinyminutes.com/docs/docker/",
    "name": "docker"
  },
  {
    "link": "https:learnxinyminutes.com/docs/emacs/",
    "name": "emacs"
  },
  {
    "link": "https:learnxinyminutes.com/docs/fish/",
    "name": "fish"
  },
  {
    "link": "https:learnxinyminutes.com/docs/git/",
    "name": "git"
  },
  {
    "link": "https:learnxinyminutes.com/docs/jquery/",
    "name": "jquery"
  },
  {
    "link": "https:learnxinyminutes.com/docs/linker/",
    "name": "linker"
  },
  {
    "link": "https:learnxinyminutes.com/docs/make/",
    "name": "make"
  },
  {
    "link": "https:learnxinyminutes.com/docs/mercurial/",
    "name": "Mercurial"
  },
  {
    "link": "https:learnxinyminutes.com/docs/messagepack/",
    "name": "messagepack"
  },
  {
    "link": "https:learnxinyminutes.com/docs/opencv/",
    "name": "OpenCV"
  },
  {
    "link": "https:learnxinyminutes.com/docs/opengl/",
    "name": "OpenGL"
  },
  {
    "link": "https:learnxinyminutes.com/docs/p5/",
    "name": "p5"
  },
  {
    "link": "https:learnxinyminutes.com/docs/powershell/",
    "name": "powershell"
  },
  {
    "link": "https:learnxinyminutes.com/docs/pyqt/",
    "name": "PyQT"
  },
  {
    "link": "https:learnxinyminutes.com/docs/qt/",
    "name": "Qt Framework"
  },
  {
    "link": "https:learnxinyminutes.com/docs/raylib/",
    "name": "raylib"
  },
  {
    "link": "https:learnxinyminutes.com/docs/ruby-ecosystem/",
    "name": "ruby ecosystem"
  },
  {
    "link": "https:learnxinyminutes.com/docs/shutit/",
    "name": "ShutIt"
  },
  {
    "link": "https:learnxinyminutes.com/docs/pythonstatcomp/",
    "name": "Statistical Computing with Python"
  },
  {
    "link": "https:learnxinyminutes.com/docs/tcsh/",
    "name": "tcsh"
  },
  {
    "link": "https:learnxinyminutes.com/docs/tmux/",
    "name": "tmux"
  },
  {
    "link": "https:learnxinyminutes.com/docs/vim/",
    "name": "vim"
  },
  {
    "link": "https:learnxinyminutes.com/docs/zfs/",
    "name": "zfs"
  }
]
const cheerio = require('cheerio');
const baseUrl = "https://learnxinyminutes.com";

let myLinksAndNames = {}
async function run(linkObj){
  try{
    const response = await fetch(linkObj.link);
    const body = await response.text();

    const $ = cheerio.load(body);

    const aTag = $('.filelink>a');
    const info = linkObj.name + "\n" + baseUrl + aTag.attr('href') + "\n\n";
    console.log(info)
    fs.appendFileSync("myData.txt", info)

  } catch{
    console.log("could no process" + linkObj.name)
  }
};


arr.forEach((el) => {
  run(el)
})
