#
# ~/.bashrc
#

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

alias ls='ls --color=auto'
alias grep='grep --color=auto'
PS1='[\u@\h \W]\$ '

# Created by `pipx` on 2024-09-04 14:14:33
export PATH="$PATH:/home/arch/.local/bin"


. "$HOME/.cargo/env"
export ANDROID_SDK_ROOT=$HOME/Projects/Android
export PATH=$PATH:$ANDROID_SDK_ROOT/tools/:$ANDROID_SDK_ROOT/cmdline-tools/latest/bin/:$ANDROID_SDK_ROOT/emulator/:$ANDROID_SDK_ROOT/platform-tools/
export ANDROID_HOME=$HOME/Projects/Android
