ECHO OFF
SET file=%1
SET type=.rs
SET ex=.exe
SET comp=%file%%type%
SET run=%file%%ex%
rustc %comp%
%run%
DEL *.exe
DEL *.pdb
PAUSE