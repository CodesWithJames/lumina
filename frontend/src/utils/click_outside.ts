
export function click_outside(node: Node, callback: () => void) {
    function handle_click(e: MouseEvent) {
        if (node.contains(e.target as Node)) {
            return
        }
        callback()
    }
    document.addEventListener('click', handle_click, { capture: false })
    return {
        destroy() {
            document.removeEventListener('click', handle_click, { capture: false })
        }
    }
}