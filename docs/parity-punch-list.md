# Parity punch list (local branch findings, 2026-07-06)

Found while playing against a local LandSandBoat server on macOS. Each item
verified by hand; headless repro harness available (`--headless` + JSON stdin,
see `ffxi-event/examples/event-probe.rs` for the event chain prober).

## Open

- **Weapon model not refreshed on live equip change.** Weapons render fine at
  character load (main/sub/ranged flow into the `load_pc` equipment mesh list
  via `look_resolver`), but equipping in-session never re-dispatches the model
  build — the weapon appears only after relog. Likely the equip packet updates
  inventory without touching the entity look / `EntityModel` signature path.

- **Run animation persists after movement stops.** Stop moving → the self
  character keeps the run cycle instead of returning to idle. Anim state machine
  misses the stop transition (server anim byte lag or local intent not checked).

- **Verify offhand/shield (sub slot) renders after relog** the way main hand
  does. `load_pc` receives `sub_weapon` separately and discards it
  (`let _ = sub_weapon;` at ffxi_actor_render.rs:204), but sub also rides the
  equipment array — unconfirmed which wins on a real shield.

- **Zone-in event lock at every login for fresh characters** (LSB fires the
  nation opening cutscene while playtime is low; with the ReqSet family now
  implemented most such events should drive — verify a brand-new character's
  first login plays or cleanly skips the opening event instead of pinning).

## Fixed on this branch

- Event VM: ReqSet/GetReqStatus family (0x27/0x28/0x29/0x2A) implemented as
  instantly-complete no-ops — real dialog text now reaches the player
  (verified: Northern San d'Oria / Letterare / event 660).
- Undriveable events auto-release with a chat notice instead of pinning the
  character InEvent behind an empty dialog.
- Self speed syncs from CHAR_PC packets (GM !speed now applies locally).
- Compact2 preset: A/D strafe + Q/E turn; camera rotation inverted both axes
  (local preference).
