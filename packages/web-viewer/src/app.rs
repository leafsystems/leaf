use crate::api::get_data;
use crate::util::handle_future;
use common_interfaces::GatewayEvent;
use yew::{prelude::*, virtual_dom::VNode};
use yew_functional::function_component;
use yew_functional::use_state;

#[function_component(MyApp)]
pub fn my_app() -> Html {
    let (content, set_content) = use_state(|| vec![]);

    let new_content = content.clone();

    crate::hooks::use_websocket("ws://127.0.0.1:9001/updates", move |data: GatewayEvent| {
        log::info!("Value was updated!");
        let mut new_content: Vec<GatewayEvent> = content.as_ref().clone();
        new_content.push(data);
        set_content(new_content.into());
    });

    let list = html! {<entry_list::GatewayEntryList entries=new_content />};

    let temp_display = html! {
        <div class="flex flex-row items-center justify-center mt-6">
            <div class="font-medium text-6xl">{"24°"}</div>
            <div class="flex flex-col items-center ml-6">
                <div>{"Cloudy"}</div>
                <div class="mt-1">
                    <span class="text-sm"><i class="far fa-long-arrow-up"></i></span>
                    <span class="text-sm font-light text-gray-500">{"28°C"}</span>
                </div>
                <div>
                    <span class="text-sm"><i class="far fa-long-arrow-down"></i></span>
                    <span class="text-sm font-light text-gray-500">{"20°C"}</span>
                </div>
            </div>
        </div>
    };

    html! {
        <div class="flex items-center justify-center flex-col" >
            <div class="flex items-center justify-center">
                <div class="flex flex-col bg-white rounded p-4 w-full max-w-xs">

                    <div class="font-bold text-xl">{"LEAF Systems Indoor GPS 🛠"}</div>
                    <div class="text-sm text-gray-500">{"Thursday 10 May 2020"}</div>

                    <div
                        class="mt-6 text-6xl self-center inline-flex items-center justify-center rounded-lg text-indigo-400 h-24 w-24">
                        <icons::CloudIcon />
                    </div>
                    {temp_display}
                    <div class="flex flex-row justify-between mt-6">
                        <info_card::InfoCard title="Wind" content="9k/h" />
                        <info_card::InfoCard title="Humdity" content="68%" />
                        <info_card::InfoCard title="Visibility" content="10km" />
                    </div>
                </div>
            </div>
            {list}
        </div>
    }
}

mod entry_list {
    use std::{rc::Rc, time::UNIX_EPOCH};

    use super::*;

    #[derive(Debug, Properties, PartialEq, Clone)]
    pub struct GatewayEntryListProps {
        pub entries: Rc<Vec<GatewayEvent>>,
    }

    #[function_component(GatewayEntryList)]
    pub fn entry_list(props: &GatewayEntryListProps) -> Html {
        let titles = ["Name", "Time", "Status"]
            .into_iter()
            .map(|entry| {
                html! {
                    <th class="px-16 py-2">
                        <span class="text-gray-300">{entry}</span>
                    </th>
                }
            })
            .collect::<Vec<_>>();

        let entries = props
            .entries
            .iter()
            .enumerate()
            .map(|(idx, entry)| {
                html! {
                    <GatewayEntry entry=entry key=idx/>
                }
            })
            .rev()
            .collect::<Vec<_>>();

        html! {
            <table class="table-auto">
            // <table class="min-w-full table-auto">
                <thead class="justify-between">
                    <tr class="bg-gray-800">
                        {titles}
                    </tr>
                </thead>
                <tbody class="bg-gray-200">
                    {entries}
                </tbody>
            </table>
        }
    }

    #[derive(Debug, Properties, PartialEq, Clone)]
    pub struct GatewayEntryProps {
        pub entry: GatewayEvent,
    }

    #[function_component(GatewayEntry)]
    pub fn gateway_entry(props: &GatewayEntryProps) -> Html {
        let GatewayEntryProps { entry } = props;
        let t = std::time::Duration::from_millis(entry.time as u64);
        let d = UNIX_EPOCH + t;

        let datetime = chrono::DateTime::<chrono::Utc>::from(d);
        // // Formats the combined date and time with the specified format string.
        let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        html! {
              <tr class="bg-white border-4 border-gray-200">
                // <td class="px-16 py-2 flex flex-row items-center">
                //     <img class="h-8 w-8 rounded-full object-cover " src="https://randomuser.me/api/portraits/men/30.jpg"
                //         alt="" />
                // </td>
                <td>
                    <span class="text-center ml-2 font-semibold">{"Mobile Tag: 0001"}</span>
                </td>

                // <td class="px-16 py-2">
                //     <button
                //         class="bg-indigo-500 text-white px-4 py-2 border rounded-md hover:bg-white hover:border-indigo-500 hover:text-black ">
                //         {"Open Link"}
                //     </button>
                // </td>

                <td class="px-16 py-2">
                    <span>{timestamp_str}</span>
                </td>


                <td class="px-16 py-2">
                    <span class="text-green-500">
                        <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h5 " viewBox="0 0 24 24" stroke-width="1.5"
                            stroke="#2c3e50" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z" />
                            <path d="M5 12l5 5l10 -10" />
                        </svg>
                    </span>
                </td>
            </tr>
        }
    }
}

mod icons {
    use super::*;

    #[function_component(CloudIcon)]
    pub fn cloud_icon() -> Html {
        html! {
            <svg class="w-32 h-32" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z">
                </path>
            </svg>
        }
    }
}

mod info_card {
    use super::*;
    #[derive(Debug, Properties, PartialEq, Clone)]
    pub struct InfoCardProps {
        pub title: String,
        pub content: String,
    }

    #[function_component(InfoCard)]
    pub fn info_card(props: &InfoCardProps) -> Html {
        let InfoCardProps { content, title } = props;
        html! {
            <div class="flex flex-col items-center">
                <div class="font-medium text-sm">{title}</div>
                <div class="text-sm text-gray-500">{content}</div>
            </div>
        }
    }
}
