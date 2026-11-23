# Windows active keyboard focus (Gold S11)

Branch `gold-window-a11y-6ce2b837`, based on Gold's existing fork revision
`6ce2b8375a5e1755ac8b11af5f17c4b9b86260c5`. Do not change the existing input-fix
branches or `dev`.

Windows keeps a thread's keyboard-focus window separately from activation. A
window created in the background can retain keyboard focus; bringing it forward
can produce WM_NCACTIVATE without another WM_SETFOCUS. winit 0.30.13 tracks both
states and emits WindowEvent::Focused when their conjunction changes. Rune's
Windows accessibility adapter formerly ignored that event and listened only for
WM_SETFOCUS/WM_KILLFOCUS, leaving AccessKit's host-focus flag stale.

Use winit's combined focus at adapter creation and forward the same host focus
events the input runtime consumes. Remove the competing native-focus listener so
an inactive window retaining keyboard focus cannot be reported focused. Raise
AccessKit's queued events after releasing the RefCell borrow, allowing nested
WM_GETOBJECT. The subclass still handles UI Automation's provider requests.

Gold's native fail-first and confirmation live in
`ui-rune/scripts/Prove-WindowAccessibility.ps1`, mode `window-a11y-focus`. It opens
Gold behind a proof-owned foreground window, reads thread active/focus handles,
activates once, and requires HasKeyboardFocus plus literal UIA SetValue without
minimize/restore or focus reacquisition. Native qualification is pending.

This restores expected native focus reporting; no deliberate product divergence
from Original is intended. No public publication is authorized by this file.
