# fabrewk completion for bash, and for zsh with bashcompinit; generated from the declaration of commands.
_fabrewk() {
    eval '_fabrewk_command=${COMP_WORDS[1]}'
    if [ "$COMP_CWORD" -eq 1 ]; then
        _fabrewk_words='status config check adopt propose authorize reject run --help --version'
    else
        case $_fabrewk_command in
        status) _fabrewk_words='--json --help' ;;
        config) _fabrewk_words='--json --help project.name languages.canonicalDocumentationLanguage languages.documentationLanguages languages.codeLanguage languages.evidenceLanguage languages.delegationLanguage languages.agentInteraction languages.interfaceLanguage maintainer.alias environment.shell limits.rounds limits.dispatches limits.minutes review.differentProvider gate.protectedPaths git.signOff' ;;
        check) _fabrewk_words='--json --help' ;;
        adopt) _fabrewk_words='--say --say-lang --widen --json --help' ;;
        propose) _fabrewk_words='--json --help' ;;
        authorize) _fabrewk_words='--seen --say --say-lang --widen --run --json --help' ;;
        reject) _fabrewk_words='--say --say-lang --json --help' ;;
        run) _fabrewk_words='--say --say-lang --json --help' ;;
        --version) _fabrewk_words='--json' ;;
        *) _fabrewk_words='' ;;
        esac
    fi
    eval 'COMPREPLY=($(compgen -W "$_fabrewk_words" -- "$2"))'
}
complete -F _fabrewk fabrewk
