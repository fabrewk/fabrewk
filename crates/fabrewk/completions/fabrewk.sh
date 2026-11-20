# fabrewk completion for bash, and for zsh with bashcompinit; generated from the declaration of commands.
_fabrewk() {
    eval '_fabrewk_command=${COMP_WORDS[1]}'
    if [ "$COMP_CWORD" -eq 1 ]; then
        _fabrewk_words='status config check assess apply onboard adopt propose authorize accept reject run sweep discard approve close --help --version'
    else
        case $_fabrewk_command in
        status) _fabrewk_words='--json --help' ;;
        config) _fabrewk_words='--json --help project.name languages.canonicalDocumentationLanguage languages.documentationLanguages languages.codeLanguage languages.evidenceLanguage languages.delegationLanguage languages.agentInteraction languages.interfaceLanguage maintainer.alias environment.shell limits.rounds limits.dispatches limits.minutes limits.freeDiskMegabytes review.differentProvider gate.protectedPaths gate.protectedNames git.signOff' ;;
        check) _fabrewk_words='--json --help' ;;
        assess) _fabrewk_words='--mode --publication --alias --key --tool --lang --purpose --audience --excluded --json --help' ;;
        apply) _fabrewk_words='--mode --publication --alias --key --tool --lang --purpose --audience --excluded --say --say-lang --json --help' ;;
        onboard) _fabrewk_words='--help' ;;
        adopt) _fabrewk_words='--say --say-lang --widen --json --help' ;;
        propose) _fabrewk_words='--json --help' ;;
        authorize) _fabrewk_words='--seen --say --say-lang --widen --run --json --help' ;;
        accept) _fabrewk_words='--seen --say --say-lang --json --help' ;;
        reject) _fabrewk_words='--say --say-lang --json --help' ;;
        run) _fabrewk_words='--say --say-lang --json --help' ;;
        sweep) _fabrewk_words='--answer --json --help' ;;
        discard) _fabrewk_words='--say --say-lang --json --help' ;;
        approve) _fabrewk_words='--say --say-lang --json --help' ;;
        close) _fabrewk_words='--say --say-lang --json --help' ;;
        --version) _fabrewk_words='--json' ;;
        *) _fabrewk_words='' ;;
        esac
    fi
    eval 'COMPREPLY=($(compgen -W "$_fabrewk_words" -- "$2"))'
}
complete -F _fabrewk fabrewk
