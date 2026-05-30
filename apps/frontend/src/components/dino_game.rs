use leptos::prelude::*;

use crate::i18n::{use_language, Language};

/// Canvas-based cyberpunk runner mini-game shown on every 5th 404 visit.
#[component]
pub fn DinoGame() -> impl IntoView {
    let lang = use_language();

    let start_label = move || match lang.get() {
        Language::Fr => "Appuie sur Espace ou clique pour démarrer",
        Language::En => "Press Space or click to start",
    };
    let gameover_label = move || match lang.get() {
        Language::Fr => "GAME OVER",
        Language::En => "GAME OVER",
    };
    let restart_label = move || match lang.get() {
        Language::Fr => "Espace / clic pour rejouer",
        Language::En => "Space / click to restart",
    };
    let score_label = move || match lang.get() {
        Language::Fr => "Score",
        Language::En => "Score",
    };
    let best_label = move || match lang.get() {
        Language::Fr => "Meilleur",
        Language::En => "Best",
    };

    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    #[cfg(feature = "hydrate")]
    {
        use std::cell::RefCell;
        use std::rc::Rc;
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        #[derive(Clone, Copy, PartialEq)]
        enum GameState {
            Idle,
            Running,
            GameOver,
        }

        struct Obstacle {
            x: f64,
            width: f64,
            height: f64,
        }

        struct GameData {
            state: GameState,
            score: f64,
            high_score: f64,
            // Player: y is offset from ground (0 = standing)
            player_y: f64,
            player_vy: f64,
            obstacles: Vec<Obstacle>,
            speed: f64,
            frame_count: u64,
            last_time: f64,
        }

        const CANVAS_W: f64 = 700.0;
        const CANVAS_H: f64 = 200.0;
        const GROUND_Y: f64 = 160.0; // y of the ground line
        const PLAYER_X: f64 = 60.0;
        const PLAYER_W: f64 = 28.0;
        const PLAYER_H: f64 = 44.0;
        const GRAVITY: f64 = 0.6;
        const JUMP_VY: f64 = -13.0;
        const BASE_SPEED: f64 = 2.0; // slow start
        const SPEED_INC: f64 = 0.002; // gradual acceleration per frame (x2 original)

        fn load_high_score() -> f64 {
            let window = web_sys::window().unwrap();
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(val)) = storage.get_item("not_found_high_score") {
                    return val.parse::<f64>().unwrap_or(0.0);
                }
            }
            0.0
        }

        fn save_high_score(score: f64) {
            let window = web_sys::window().unwrap();
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("not_found_high_score", &score.to_string());
            }
        }

        fn new_game_data(high_score: f64) -> GameData {
            GameData {
                state: GameState::Idle,
                score: 0.0,
                high_score,
                player_y: 0.0,
                player_vy: 0.0,
                obstacles: vec![],
                speed: BASE_SPEED,
                frame_count: 0,
                last_time: 0.0,
            }
        }

        // Draw the full frame onto the canvas context
        fn draw(ctx: &web_sys::CanvasRenderingContext2d, data: &GameData) {
            // Background
            ctx.set_fill_style_str("#0a0a0f");
            ctx.fill_rect(0.0, 0.0, CANVAS_W, CANVAS_H);

            // Scanlines
            ctx.set_stroke_style_str("rgba(0,255,220,0.04)");
            ctx.set_line_width(1.0);
            let mut sy = 0.0_f64;
            while sy < CANVAS_H {
                ctx.begin_path();
                ctx.move_to(0.0, sy);
                ctx.line_to(CANVAS_W, sy);
                ctx.stroke();
                sy += 4.0;
            }

            // Ground line
            ctx.set_stroke_style_str("#00ffdc");
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(0.0, GROUND_Y);
            ctx.line_to(CANVAS_W, GROUND_Y);
            ctx.stroke();

            // Obstacles (neon red)
            ctx.set_fill_style_str("#ff2244");
            ctx.set_shadow_color("#ff2244");
            ctx.set_shadow_blur(8.0);
            for obs in &data.obstacles {
                let top = GROUND_Y - obs.height;
                ctx.fill_rect(obs.x, top, obs.width, obs.height);
            }
            ctx.set_shadow_blur(0.0);

            // Player robot (neon cyan)
            let px = PLAYER_X;
            let py = GROUND_Y - PLAYER_H - data.player_y;

            ctx.set_fill_style_str("#00ffdc");
            ctx.set_shadow_color("#00ffdc");
            ctx.set_shadow_blur(10.0);

            // Body
            ctx.fill_rect(px, py + 16.0, PLAYER_W, 20.0);
            // Head
            ctx.fill_rect(px + 3.0, py + 4.0, 22.0, 14.0);
            // Antenna
            ctx.fill_rect(px + 12.0, py, 4.0, 6.0);
            ctx.fill_rect(px + 10.0, py - 3.0, 8.0, 4.0);
            // Eyes (dark cutout)
            ctx.set_fill_style_str("#0a0a0f");
            ctx.fill_rect(px + 6.0, py + 7.0, 5.0, 5.0);
            ctx.fill_rect(px + 15.0, py + 7.0, 5.0, 5.0);
            ctx.set_fill_style_str("#00ffdc");

            // Legs (animate based on frame count)
            let leg_anim = if data.player_y > 0.0 {
                // In the air — legs together
                (0.0_f64, 0.0_f64)
            } else {
                let phase = (data.frame_count / 12) % 2;
                if phase == 0 {
                    (8.0, 0.0)
                } else {
                    (0.0, 8.0)
                }
            };
            ctx.fill_rect(px + 4.0, py + 36.0, 8.0, leg_anim.0 + 2.0);
            ctx.fill_rect(px + 16.0, py + 36.0, 8.0, leg_anim.1 + 2.0);

            ctx.set_shadow_blur(0.0);

            // Score text
            ctx.set_fill_style_str("#00ffdc");
            ctx.set_font("bold 14px monospace");
            let score_str = format!("Score: {:05}", data.score as u64);
            let best_str = format!("Best: {:05}", data.high_score as u64);
            let _ = ctx.fill_text(&score_str, CANVAS_W - 160.0, 20.0);
            let _ = ctx.fill_text(&best_str, CANVAS_W - 160.0, 38.0);

            // Overlays
            match data.state {
                GameState::Idle => {
                    // Semi-transparent overlay
                    ctx.set_fill_style_str("rgba(0,0,0,0.55)");
                    ctx.fill_rect(0.0, 0.0, CANVAS_W, CANVAS_H);
                    ctx.set_fill_style_str("#00ffdc");
                    ctx.set_font("bold 18px monospace");
                    let _ = ctx.fill_text("[ CYBER RUNNER ]", CANVAS_W / 2.0 - 100.0, CANVAS_H / 2.0 - 14.0);
                    ctx.set_font("13px monospace");
                    ctx.set_fill_style_str("#aaaaaa");
                    let _ = ctx.fill_text("Press Space or click to start", CANVAS_W / 2.0 - 115.0, CANVAS_H / 2.0 + 12.0);
                }
                GameState::GameOver => {
                    ctx.set_fill_style_str("rgba(0,0,0,0.55)");
                    ctx.fill_rect(0.0, 0.0, CANVAS_W, CANVAS_H);
                    ctx.set_fill_style_str("#ff2244");
                    ctx.set_font("bold 22px monospace");
                    let _ = ctx.fill_text("GAME OVER", CANVAS_W / 2.0 - 70.0, CANVAS_H / 2.0 - 16.0);
                    ctx.set_fill_style_str("#00ffdc");
                    ctx.set_font("14px monospace");
                    let final_score = format!("Score: {}  |  Best: {}", data.score as u64, data.high_score as u64);
                    let _ = ctx.fill_text(&final_score, CANVAS_W / 2.0 - 110.0, CANVAS_H / 2.0 + 8.0);
                    ctx.set_fill_style_str("#aaaaaa");
                    ctx.set_font("13px monospace");
                    let _ = ctx.fill_text("Space / click to restart", CANVAS_W / 2.0 - 96.0, CANVAS_H / 2.0 + 30.0);
                }
                GameState::Running => {}
            }
        }

        Effect::new(move |_| {
            let canvas_el = match canvas_ref.get() {
                Some(c) => c,
                None => return,
            };

            canvas_el.set_width(CANVAS_W as u32);
            canvas_el.set_height(CANVAS_H as u32);

            let ctx: web_sys::CanvasRenderingContext2d = canvas_el
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();

            let high_score = load_high_score();
            let data = Rc::new(RefCell::new(new_game_data(high_score)));

            // Draw initial idle frame
            draw(&ctx, &data.borrow());

            // ---- rAF loop ----
            // RwSignal is Copy + Send + Sync, so it can be captured by on_cleanup.
            let raf_handle: RwSignal<Option<i32>> = RwSignal::new(None);

            let data_raf = data.clone();
            let ctx_raf = ctx.clone();

            let window = web_sys::window().unwrap();

            // We use a Rc<RefCell<Option<Closure>>> trick so the closure can schedule itself
            let raf_closure: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> =
                Rc::new(RefCell::new(None));
            let raf_closure_clone = raf_closure.clone();

            *raf_closure.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
                let mut d = data_raf.borrow_mut();

                if d.state == GameState::Running {
                    let dt = if d.last_time == 0.0 {
                        16.0
                    } else {
                        (timestamp - d.last_time).min(50.0)
                    };
                    d.last_time = timestamp;

                    // Speed ramp
                    d.speed += SPEED_INC * dt / 16.0;
                    d.score += d.speed * dt / 16.0 / 6.0;
                    d.frame_count += 1;

                    // Player physics
                    d.player_vy += GRAVITY;
                    d.player_y -= d.player_vy;
                    if d.player_y <= 0.0 {
                        d.player_y = 0.0;
                        d.player_vy = 0.0;
                    }

                    // Move obstacles
                    let spd = d.speed;
                    for obs in d.obstacles.iter_mut() {
                        obs.x -= spd;
                    }
                    d.obstacles.retain(|o| o.x + o.width > -10.0);

                    // Spawn obstacle: every ~70-120 frames, scaled by speed
                    let spawn_interval = ((120.0 - d.speed * 4.0).max(50.0)) as u64;
                    if d.frame_count % spawn_interval == 0 {
                        use js_sys::Math;
                        let h = 24.0 + Math::random() * 32.0;
                        let w = 14.0 + Math::random() * 18.0;
                        d.obstacles.push(Obstacle {
                            x: CANVAS_W + 10.0,
                            width: w,
                            height: h,
                        });
                    }

                    // Collision AABB
                    let px = PLAYER_X;
                    let py = GROUND_Y - PLAYER_H - d.player_y;
                    let margin = 4.0; // slight forgiveness
                    for obs in &d.obstacles {
                        let ox = obs.x;
                        let oy = GROUND_Y - obs.height;
                        if px + PLAYER_W - margin > ox
                            && px + margin < ox + obs.width
                            && py + PLAYER_H - margin > oy
                            && py + margin < oy + obs.height
                        {
                            d.state = GameState::GameOver;
                            if d.score > d.high_score {
                                d.high_score = d.score;
                                save_high_score(d.high_score);
                            }
                            break;
                        }
                    }
                }

                draw(&ctx_raf, &d);
                drop(d);

                // Schedule next frame
                let win = web_sys::window().unwrap();
                if let Some(cb) = raf_closure_clone.borrow().as_ref() {
                    let id = win
                        .request_animation_frame(cb.as_ref().unchecked_ref())
                        .unwrap();
                    raf_handle.set(Some(id));
                }
            }) as Box<dyn FnMut(f64)>));

            // Kick off the loop
            {
                let id = window
                    .request_animation_frame(
                        raf_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
                    )
                    .unwrap();
                raf_handle.set(Some(id));
            }

            // ---- Input handlers ----
            let jump_or_start = {
                let data_input = data.clone();
                move || {
                    let mut d = data_input.borrow_mut();
                    match d.state {
                        GameState::Idle => {
                            d.state = GameState::Running;
                            d.last_time = 0.0;
                        }
                        GameState::Running => {
                            if d.player_y <= 0.0 {
                                d.player_vy = JUMP_VY;
                            }
                        }
                        GameState::GameOver => {
                            let hs = d.high_score;
                            *d = new_game_data(hs);
                            d.state = GameState::Running;
                        }
                    }
                }
            };

            // Keydown
            {
                let jump_fn = jump_or_start.clone();
                let keydown_cb = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::wrap(Box::new(
                    move |e: web_sys::KeyboardEvent| {
                        let key = e.key();
                        if key == " " || key == "ArrowUp" || key == "w" || key == "W" {
                            e.prevent_default();
                            jump_fn();
                        }
                    },
                ));
                window
                    .add_event_listener_with_callback(
                        "keydown",
                        keydown_cb.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                keydown_cb.forget();
            }

            // Click
            {
                let jump_fn = jump_or_start.clone();
                let click_cb = Closure::<dyn FnMut(web_sys::MouseEvent)>::wrap(Box::new(
                    move |_e: web_sys::MouseEvent| {
                        jump_fn();
                    },
                ));
                canvas_el
                    .add_event_listener_with_callback(
                        "click",
                        click_cb.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                click_cb.forget();
            }

            // Touchstart
            {
                let jump_fn = jump_or_start.clone();
                let touch_cb = Closure::<dyn FnMut(web_sys::TouchEvent)>::wrap(Box::new(
                    move |e: web_sys::TouchEvent| {
                        e.prevent_default();
                        jump_fn();
                    },
                ));
                canvas_el
                    .add_event_listener_with_callback(
                        "touchstart",
                        touch_cb.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                touch_cb.forget();
            }

            // Cleanup: cancel rAF when component unmounts (via on_cleanup).
            // raf_handle is RwSignal (Copy + Send + Sync) so on_cleanup accepts it.
            on_cleanup(move || {
                if let Some(id) = raf_handle.get_untracked() {
                    let _ = web_sys::window().map(|w| w.cancel_animation_frame(id));
                }
            });
        });
    }

    view! {
        <div class="relative w-full max-w-[700px] mx-auto">
            <canvas
                node_ref=canvas_ref
                class="w-full rounded border border-border bg-[#0a0a0f] touch-none select-none cursor-pointer"
                aria-label=move || match lang.get() {
                    Language::Fr => "Jeu de course cyberpunk",
                    Language::En => "Cyberpunk runner game",
                }
                role="img"
            />
            // Fallback labels kept in DOM for screen-readers / SSR
            <p class="sr-only">{start_label}</p>
            <p class="sr-only">{gameover_label}</p>
            <p class="sr-only">{restart_label}</p>
            <p class="sr-only">{score_label}</p>
            <p class="sr-only">{best_label}</p>
        </div>
    }
}
