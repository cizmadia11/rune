# Windows active keyboard focus (Gold S11)

Branch `gold-window-a11y-6ce2b837`, based on Gold's existing fork revision
`6ce2b8375a5e1755ac8b11af5f17c4b9b86260c5`. Existing input-fix branches and
`dev` are unchanged. Publication requires the consumer lead's explicit clearance.

Windows keeps a thread's active and keyboard-focus windows even while another
process is foreground. Bringing the process forward can produce WM_ACTIVATEAPP
without another WM_SETFOCUS. Rune's adapter listened only for WM_SETFOCUS and
WM_KILLFOCUS, leaving AccessKit's host-focus flag stale after this first activation.

Handle process activation as well as native keyboard-focus messages, reading
both the real foreground HWND and this thread's GetFocus HWND. Reconcile those
readings at tree-update time, so a nested message that could not borrow the
adapter cannot leave a stale focus flag. Raise queued focus events after the
RefCell borrow ends and before applying tree changes.

Forwarding winit's Focused events alone was investigated in local commit
b557d28b; winit's is_active/is_focused state is local to the input queue and does
not establish foreground ownership. That approach is superseded, preserving
history and the preexisting native focus integration.

Gold's native regression is `ui-rune/scripts/Prove-WindowAccessibility.ps1`,
mode `window-a11y-focus`. It opens behind a proof-owned foreground window,
requires the background window and field to report no UIA keyboard focus,
activates Gold once, and requires HasKeyboardFocus plus an exact literal UIA
SetValue without minimize/restore or focus reacquisition. Literal entry happens
after activation because managed UIA Focus and Value actions can activate a
background window themselves.

Original 6ce2b837 fails first activation (`s11-w1-focus-failfirst-1`, exit 1).
The native confirmation (`s11-w1-focus-local-3`, Gold 99ffe9eb, exit 0) uses the
exact 83550ef7 source archive and a SHA-256-guarded private Gold image: background
focus false, first-activation focus true, exact literal value, no minimize/restore.

This restores expected native focus reporting; no deliberate product divergence
from Original is intended. Gold's handoff records qualifications and remaining
startup/header findings; those are separate from this framework fix.
