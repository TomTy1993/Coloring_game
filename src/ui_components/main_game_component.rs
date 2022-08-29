use yew::prelude::*;
use crate::logic::game_manager::GameManager;
use crate::logic::colors::Colors;
use crate::logic::game_manager::GameState;
use crate::ui_components::color_button_component::ColorButtonComponent;


pub enum Msg {
    ButtonPressed(Colors),
    ResetGame
}

pub struct MainGameComponent {
    game_manager: GameManager
}

impl Component for MainGameComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game_manager: GameManager::new(21, 20, 20)
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Msg::ButtonPressed(color) = msg {
            return self.game_manager.make_move(&color);
        } 
        else {
            self.game_manager.reset_game();
            return true;
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                {
                    self.create_title(ctx)
                }
                <div>
                    {
                        self.create_table()
                    }
                </div>
                <div>
                    <div class="buttons">
                        {
                            self.create_buttons(ctx)
                        }
                    </div>
                </div>
                {
                    self.create_end_screen(ctx)
                }
            </div>
        }
    }
}

impl MainGameComponent {
    fn create_title(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="center_content">
                <h1 class="title">
                { format!("Numbers of tries left: {}", self.game_manager.tryies_left) }
                </h1>
                <button class="retry_button" onclick={ctx.link().callback(|_| Msg::ResetGame)}>{ "Reset" }</button>
            </div>
        }
    }

    fn create_table(&self) -> Html {
        html! {
            <table cellspacing="0" cellpadding="0">
                {
                    self.game_manager.board.state.iter().map(Self::create_row).collect::<Html>()
                }
            </table>
        }
    }

    fn create_row(cells_row: &Vec<Colors>) -> Html{
        html!{
            <tr>
            {
                cells_row.iter().map(Self::create_cell).collect::<Html>()
            }
            </tr>
        }
    }

    fn create_cell(cell_color: &Colors) -> Html{
        match cell_color {
            Colors::Red => html!(<td class="cell red_cell"></td>),
            Colors::Green => html!(<td class="cell green_cell"></td>),
            Colors::Blue => html!(<td class="cell blue_cell"></td>),
            Colors::Yellow => html!(<td class="cell yellow_cell"></td>),
        }
    }

    fn create_buttons(&self, ctx: &Context<Self>) -> Html{
        [Colors::Red, Colors::Green, Colors::Blue, Colors::Yellow]
        .iter()
        .map(|color| {
            self.create_button(ctx, color)
        }).collect::<Html>()
    }

    fn create_button(&self, ctx: &Context<Self>, button_color: &Colors) -> Html{
        let link = ctx.link();
        html!(
            <ColorButtonComponent
            color={*button_color}
            is_enabled={self.game_manager.last_colos_used == *button_color} 
            on_click={link.callback(|x| Msg::ButtonPressed(x))} />
        )
    }

    fn create_end_screen(&self, ctx: &Context<Self>) -> Html{
        if self.game_manager.game_state == GameState::Won || self.game_manager.game_state == GameState::Lost{
            html! { 
                <div id="overlay">
                    <h1 class="end_game center_content">{format!("You have: {:?}", self.game_manager.game_state)}</h1>
                    <button class="center_content" onclick={ctx.link().callback(|_| Msg::ResetGame)}>{ "Reset" }</button>
                </div>
             }
        }
        else{
            html! {}
        }
    }
}