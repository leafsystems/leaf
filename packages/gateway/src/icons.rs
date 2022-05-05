use dioxus::prelude::*;

pub static Settings: Component<()> = |cx| {
    cx.render(rsx! {
        svg { class: "text-gray-600 w-5 h-5", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 20 20", fill: "none",
            path {
                fill: "currentColor",
                d: "M17.7666 7.9583L16.1916 7.4333L16.9333 5.94996C17.0085 5.7947 17.0336 5.61993 17.0053 5.44977C16.9769 5.27961 16.8964 5.12245 16.775 4.99996L15 3.22496C14.8768 3.1017 14.7182 3.02013 14.5463 2.99173C14.3743 2.96333 14.1979 2.98953 14.0416 3.06663L12.5583 3.8083L12.0333 2.2333C11.9778 2.06912 11.8726 1.92632 11.7322 1.82475C11.5918 1.72319 11.4232 1.66792 11.25 1.66663H8.74996C8.57526 1.66618 8.40483 1.72064 8.26277 1.82233C8.12071 1.92402 8.0142 2.06778 7.9583 2.2333L7.4333 3.8083L5.94996 3.06663C5.7947 2.99145 5.61993 2.9663 5.44977 2.99466C5.27961 3.02302 5.12245 3.10349 4.99996 3.22496L3.22496 4.99996C3.1017 5.1231 3.02013 5.28177 2.99173 5.45368C2.96333 5.62558 2.98953 5.80205 3.06663 5.9583L3.8083 7.44163L2.2333 7.96663C2.06912 8.02208 1.92632 8.12732 1.82475 8.26772C1.72319 8.40812 1.66792 8.57668 1.66663 8.74996V11.25C1.66618 11.4247 1.72064 11.5951 1.82233 11.7372C1.92402 11.8792 2.06778 11.9857 2.2333 12.0416L3.8083 12.5666L3.06663 14.05C2.99145 14.2052 2.9663 14.38 2.99466 14.5502C3.02302 14.7203 3.10349 14.8775 3.22496 15L4.99996 16.775C5.1231 16.8982 5.28177 16.9798 5.45368 17.0082C5.62558 17.0366 5.80205 17.0104 5.9583 16.9333L7.44163 16.1916L7.96663 17.7666C8.02253 17.9321 8.12904 18.0759 8.2711 18.1776C8.41317 18.2793 8.58359 18.3337 8.7583 18.3333H11.2583C11.433 18.3337 11.6034 18.2793 11.7455 18.1776C11.8875 18.0759 11.9941 17.9321 12.05 17.7666L12.575 16.1916L14.0583 16.9333C14.2126 17.0066 14.3856 17.0307 14.5541 17.0024C14.7225 16.9741 14.8781 16.8947 15 16.775L16.775 15C16.8982 14.8768 16.9798 14.7182 17.0082 14.5463C17.0366 14.3743 17.0104 14.1979 16.9333 14.0416L16.1916 12.5583L17.7666 12.0333C17.9308 11.9778 18.0736 11.8726 18.1752 11.7322C18.2767 11.5918 18.332 11.4232 18.3333 11.25V8.74996C18.3337 8.57526 18.2793 8.40483 18.1776 8.26277C18.0759 8.12071 17.9321 8.0142 17.7666 7.9583ZM16.6666 10.65L15.6666 10.9833C15.4367 11.0579 15.2257 11.1816 15.0483 11.3459C14.871 11.5102 14.7315 11.711 14.6395 11.9346C14.5475 12.1582 14.5053 12.3991 14.5158 12.6406C14.5262 12.8821 14.5891 13.1185 14.7 13.3333L15.175 14.2833L14.2583 15.2L13.3333 14.7C13.1196 14.5935 12.8855 14.5342 12.6469 14.526C12.4083 14.5179 12.1707 14.5611 11.9502 14.6528C11.7298 14.7445 11.5316 14.8824 11.3691 15.0573C11.2066 15.2322 11.0835 15.44 11.0083 15.6666L10.675 16.6666H9.34996L9.01663 15.6666C8.94204 15.4367 8.81832 15.2257 8.65404 15.0483C8.48977 14.871 8.28888 14.7315 8.06531 14.6395C7.84174 14.5475 7.60084 14.5053 7.35932 14.5158C7.11779 14.5262 6.88143 14.5891 6.66663 14.7L5.71663 15.175L4.79996 14.2583L5.29996 13.3333C5.41087 13.1185 5.47373 12.8821 5.48417 12.6406C5.49461 12.3991 5.45238 12.1582 5.36041 11.9346C5.26845 11.711 5.12894 11.5102 4.95158 11.3459C4.77422 11.1816 4.56325 11.0579 4.3333 10.9833L3.3333 10.65V9.34996L4.3333 9.01663C4.56325 8.94204 4.77422 8.81832 4.95158 8.65404C5.12894 8.48977 5.26845 8.28888 5.36041 8.06531C5.45238 7.84174 5.49461 7.60084 5.48417 7.35932C5.47373 7.11779 5.41087 6.88143 5.29996 6.66663L4.82496 5.74163L5.74163 4.82496L6.66663 5.29996C6.88143 5.41087 7.11779 5.47373 7.35932 5.48417C7.60084 5.49461 7.84174 5.45238 8.06531 5.36041C8.28888 5.26845 8.48977 5.12894 8.65404 4.95158C8.81832 4.77422 8.94204 4.56325 9.01663 4.3333L9.34996 3.3333H10.65L10.9833 4.3333C11.0579 4.56325 11.1816 4.77422 11.3459 4.95158C11.5102 5.12894 11.711 5.26845 11.9346 5.36041C12.1582 5.45238 12.3991 5.49461 12.6406 5.48417C12.8821 5.47373 13.1185 5.41087 13.3333 5.29996L14.2833 4.82496L15.2 5.74163L14.7 6.66663C14.5935 6.88033 14.5342 7.11442 14.526 7.35304C14.5179 7.59165 14.5611 7.82924 14.6528 8.0497C14.7445 8.27016 14.8824 8.46835 15.0573 8.63086C15.2322 8.79337 15.44 8.9164 15.6666 8.99163L16.6666 9.32496V10.65ZM9.99996 6.66663C9.34069 6.66663 8.69623 6.86213 8.14806 7.2284C7.5999 7.59467 7.17266 8.11526 6.92036 8.72435C6.66807 9.33344 6.60206 10.0037 6.73068 10.6503C6.8593 11.2969 7.17676 11.8908 7.64294 12.357C8.10911 12.8232 8.70306 13.1406 9.34966 13.2692C9.99626 13.3979 10.6665 13.3319 11.2756 13.0796C11.8847 12.8273 12.4053 12.4 12.7715 11.8519C13.1378 11.3037 13.3333 10.6592 13.3333 9.99996C13.3333 9.11591 12.9821 8.26806 12.357 7.64294C11.7319 7.01782 10.884 6.66663 9.99996 6.66663ZM9.99996 11.6666C9.67033 11.6666 9.34809 11.5689 9.07401 11.3857C8.79993 11.2026 8.58631 10.9423 8.46016 10.6378C8.33402 10.3332 8.30101 9.99811 8.36532 9.67481C8.42963 9.35151 8.58836 9.05454 8.82145 8.82145C9.05454 8.58836 9.35151 8.42963 9.67481 8.36532C9.99811 8.30101 10.3332 8.33402 10.6378 8.46016C10.9423 8.58631 11.2026 8.79993 11.3857 9.07401C11.5689 9.34809 11.6666 9.67033 11.6666 9.99996C11.6666 10.442 11.491 10.8659 11.1785 11.1785C10.8659 11.491 10.442 11.6666 9.99996 11.6666Z"
            }
        }
    })
};

pub static Logout: Component<()> = |cx| {
    cx.render(rsx! {
        svg { class: "text-gray-600 w-5 h-5", fill: "none", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 14 18",
            path {
                fill: "currentColor",
                d: "M0.333618 8.99996C0.333618 9.22097 0.421416 9.43293 0.577696 9.58922C0.733976 9.7455 0.945938 9.83329 1.16695 9.83329H7.49195L5.57528 11.7416C5.49718 11.8191 5.43518 11.9113 5.39287 12.0128C5.35057 12.1144 5.32879 12.2233 5.32879 12.3333C5.32879 12.4433 5.35057 12.5522 5.39287 12.6538C5.43518 12.7553 5.49718 12.8475 5.57528 12.925C5.65275 13.0031 5.74492 13.0651 5.84647 13.1074C5.94802 13.1497 6.05694 13.1715 6.16695 13.1715C6.27696 13.1715 6.38588 13.1497 6.48743 13.1074C6.58898 13.0651 6.68115 13.0031 6.75862 12.925L10.0919 9.59163C10.1678 9.51237 10.2273 9.41892 10.2669 9.31663C10.3503 9.11374 10.3503 8.88618 10.2669 8.68329C10.2273 8.581 10.1678 8.48755 10.0919 8.40829L6.75862 5.07496C6.68092 4.99726 6.58868 4.93563 6.48716 4.89358C6.38564 4.85153 6.27683 4.82988 6.16695 4.82988C6.05707 4.82988 5.94826 4.85153 5.84674 4.89358C5.74522 4.93563 5.65298 4.99726 5.57528 5.07496C5.49759 5.15266 5.43595 5.2449 5.3939 5.34642C5.35185 5.44794 5.33021 5.55674 5.33021 5.66663C5.33021 5.77651 5.35185 5.88532 5.3939 5.98683C5.43595 6.08835 5.49759 6.18059 5.57528 6.25829L7.49195 8.16663H1.16695C0.945938 8.16663 0.733976 8.25442 0.577696 8.4107C0.421416 8.56698 0.333618 8.77895 0.333618 8.99996ZM11.1669 0.666626H2.83362C2.17058 0.666626 1.53469 0.930018 1.06585 1.39886C0.59701 1.8677 0.333618 2.50358 0.333618 3.16663V5.66663C0.333618 5.88764 0.421416 6.0996 0.577696 6.25588C0.733976 6.41216 0.945938 6.49996 1.16695 6.49996C1.38797 6.49996 1.59993 6.41216 1.75621 6.25588C1.91249 6.0996 2.00028 5.88764 2.00028 5.66663V3.16663C2.00028 2.94561 2.08808 2.73365 2.24436 2.57737C2.40064 2.42109 2.6126 2.33329 2.83362 2.33329H11.1669C11.388 2.33329 11.5999 2.42109 11.7562 2.57737C11.9125 2.73365 12.0003 2.94561 12.0003 3.16663V14.8333C12.0003 15.0543 11.9125 15.2663 11.7562 15.4225C11.5999 15.5788 11.388 15.6666 11.1669 15.6666H2.83362C2.6126 15.6666 2.40064 15.5788 2.24436 15.4225C2.08808 15.2663 2.00028 15.0543 2.00028 14.8333V12.3333C2.00028 12.1123 1.91249 11.9003 1.75621 11.744C1.59993 11.5878 1.38797 11.5 1.16695 11.5C0.945938 11.5 0.733976 11.5878 0.577696 11.744C0.421416 11.9003 0.333618 12.1123 0.333618 12.3333V14.8333C0.333618 15.4963 0.59701 16.1322 1.06585 16.6011C1.53469 17.0699 2.17058 17.3333 2.83362 17.3333H11.1669C11.83 17.3333 12.4659 17.0699 12.9347 16.6011C13.4036 16.1322 13.6669 15.4963 13.6669 14.8333V3.16663C13.6669 2.50358 13.4036 1.8677 12.9347 1.39886C12.4659 0.930018 11.83 0.666626 11.1669 0.666626Z"
            }
        }
    })
};

pub static Mobilemenu: Component<()> = |cx| {
    cx.render(rsx! {
        svg {
            class: "text-white bg-indigo-500 hover:bg-indigo-600 block h-8 w-8 p-2 rounded",
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            title { "Mobile menu" }
            path { d: "M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z" }
        }
    })
};

pub static ArrowDown: Component<()> = |cx| {
    cx.render(rsx! {
        span { class: "inline-block ml-auto",
            svg { class: "text-gray-400 w-3 h-3", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 10 6", fill: "none",
                path {
                    d: "M9.08329 0.666626C8.74996 0.333293 8.24996 0.333293 7.91663 0.666626L4.99996 3.58329L2.08329 0.666626C1.74996 0.333293 1.24996 0.333293 0.916626 0.666626C0.583293 0.999959 0.583293 1.49996 0.916626 1.83329L4.41663 5.33329C4.58329 5.49996 4.74996 5.58329 4.99996 5.58329C5.24996 5.58329 5.41663 5.49996 5.58329 5.33329L9.08329 1.83329C9.41663 1.49996 9.41663 0.999959 9.08329 0.666626Z",
                    fill: "currentColor"
                }
            }
        }
    })
};

pub static Odometer: Component<()> = |cx| {
    cx.render(rsx! {
        svg { class: "text-indigo-100 w-5 h-5", view_box: "0 0 18 18", fill: "none", xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M14.9066 3.12873C14.9005 3.12223 14.8987 3.11358 14.8923 3.10722C14.8859 3.10086 14.8771 3.09893 14.8706 3.09278C13.3119 1.53907 11.2008 0.666626 8.99996 0.666626C6.79914 0.666626 4.68807 1.53907 3.12935 3.09278C3.12279 3.09893 3.11404 3.10081 3.10763 3.10722C3.10122 3.11363 3.09944 3.12222 3.09334 3.12873C1.93189 4.29575 1.14217 5.78067 0.823851 7.39609C0.505534 9.01151 0.672885 10.685 1.30478 12.2054C1.93668 13.7258 3.00481 15.025 4.37435 15.9389C5.7439 16.8528 7.35348 17.3405 8.99996 17.3405C10.6464 17.3405 12.256 16.8528 13.6256 15.9389C14.9951 15.025 16.0632 13.7258 16.6951 12.2054C17.327 10.685 17.4944 9.01151 17.1761 7.39609C16.8578 5.78067 16.068 4.29575 14.9066 3.12873ZM8.99992 15.6666C8.00181 15.6663 7.01656 15.4414 6.11714 15.0087C5.21773 14.5759 4.42719 13.9464 3.80409 13.1666H7.15015C7.38188 13.4286 7.66662 13.6383 7.98551 13.782C8.3044 13.9257 8.65017 14 8.99992 14C9.34968 14 9.69544 13.9257 10.0143 13.782C10.3332 13.6383 10.618 13.4286 10.8497 13.1666H14.1958C13.5727 13.9464 12.7821 14.5759 11.8827 15.0087C10.9833 15.4414 9.99804 15.6663 8.99992 15.6666ZM8.16659 11.5C8.16659 11.3351 8.21546 11.174 8.30703 11.037C8.3986 10.8999 8.52875 10.7931 8.68102 10.7301C8.83329 10.667 9.00085 10.6505 9.1625 10.6826C9.32415 10.7148 9.47263 10.7942 9.58918 10.9107C9.70572 11.0272 9.78509 11.1757 9.81724 11.3374C9.8494 11.499 9.83289 11.6666 9.76982 11.8189C9.70675 11.9711 9.59994 12.1013 9.4629 12.1929C9.32586 12.2844 9.16474 12.3333 8.99992 12.3333C8.77898 12.3331 8.56714 12.2452 8.41091 12.089C8.25468 11.9327 8.16681 11.7209 8.16659 11.5ZM15.1751 11.5017L15.1665 11.5H11.4999C11.4983 10.9846 11.3373 10.4824 11.0389 10.0623C10.7405 9.64218 10.3193 9.32472 9.83325 9.15352V6.49996C9.83325 6.27894 9.74546 6.06698 9.58918 5.9107C9.4329 5.75442 9.22093 5.66663 8.99992 5.66663C8.77891 5.66663 8.56695 5.75442 8.41067 5.9107C8.25439 6.06698 8.16659 6.27894 8.16659 6.49996V9.15352C7.68054 9.32472 7.25939 9.64218 6.96098 10.0623C6.66256 10.4824 6.50151 10.9846 6.49992 11.5H2.83334L2.82474 11.5017C2.60799 10.9669 2.46221 10.406 2.39114 9.83329H3.16659C3.3876 9.83329 3.59956 9.74549 3.75584 9.58921C3.91212 9.43293 3.99992 9.22097 3.99992 8.99996C3.99992 8.77894 3.91212 8.56698 3.75584 8.4107C3.59956 8.25442 3.3876 8.16663 3.16659 8.16663H2.39114C2.54005 6.9821 3.00621 5.85981 3.74037 4.91838L4.28597 5.46399C4.36335 5.54137 4.4552 5.60274 4.5563 5.64462C4.65739 5.68649 4.76574 5.70804 4.87517 5.70804C4.98459 5.70804 5.09294 5.68649 5.19404 5.64461C5.29513 5.60274 5.38699 5.54136 5.46436 5.46399C5.54173 5.38661 5.60311 5.29476 5.64498 5.19366C5.68686 5.09257 5.70841 4.98422 5.70841 4.87479C5.70841 4.76537 5.68686 4.65702 5.64498 4.55592C5.60311 4.45483 5.54173 4.36297 5.46435 4.2856L4.91881 3.74005C5.86016 3.00613 6.98227 2.5401 8.16659 2.39118V3.16663C8.16659 3.38764 8.25439 3.5996 8.41067 3.75588C8.56695 3.91216 8.77891 3.99996 8.99992 3.99996C9.22093 3.99996 9.4329 3.91216 9.58918 3.75588C9.74546 3.5996 9.83325 3.38764 9.83325 3.16663V2.39118C11.0176 2.5401 12.1397 3.00613 13.081 3.74005L12.5355 4.2856C12.3792 4.44186 12.2914 4.6538 12.2914 4.87479C12.2914 5.09578 12.3792 5.30772 12.5355 5.46399C12.6917 5.62025 12.9037 5.70804 13.1247 5.70804C13.3457 5.70804 13.5576 5.62026 13.7139 5.46399L14.2595 4.91838C14.9936 5.85981 15.4598 6.9821 15.6087 8.16663H14.8333C14.6122 8.16663 14.4003 8.25442 14.244 8.4107C14.0877 8.56698 13.9999 8.77894 13.9999 8.99996C13.9999 9.22097 14.0877 9.43293 14.244 9.58921C14.4003 9.74549 14.6122 9.83329 14.8333 9.83329H15.6087C15.5376 10.406 15.3919 10.9669 15.1751 11.5017Z",
                fill: "currentColor"
            }
        }
    })
};

pub static IconCopy: Component<()> = |cx| {
    cx.render(rsx! {
        svg { class: "text-indigo-400 h-3 w-3", view_box: "0 0 12 14", xmlns: "http://www.w3.org/2000/svg", fill: "none",
            path {
                d: "M8.66667 12.3333H3.33333C2.8029 12.3333 2.29419 12.1226 1.91912 11.7476C1.54405 11.3725 1.33333 10.8638 1.33333 10.3333V3.66668C1.33333 3.48987 1.2631 3.3203 1.13807 3.19527C1.01305 3.07025 0.843478 3.00001 0.666667 3.00001C0.489856 3.00001 0.320286 3.07025 0.195262 3.19527C0.0702379 3.3203 0 3.48987 0 3.66668V10.3333C0 11.2174 0.351189 12.0652 0.976311 12.6904C1.60143 13.3155 2.44928 13.6667 3.33333 13.6667H8.66667C8.84348 13.6667 9.01305 13.5964 9.13807 13.4714C9.2631 13.3464 9.33333 13.1768 9.33333 13C9.33333 12.8232 9.2631 12.6536 9.13807 12.5286C9.01305 12.4036 8.84348 12.3333 8.66667 12.3333ZM4.66667 7.66668C4.66667 7.84349 4.7369 8.01306 4.86193 8.13808C4.98695 8.26311 5.15652 8.33334 5.33333 8.33334H8.66667C8.84348 8.33334 9.01305 8.26311 9.13807 8.13808C9.2631 8.01306 9.33333 7.84349 9.33333 7.66668C9.33333 7.48987 9.2631 7.3203 9.13807 7.19527C9.01305 7.07025 8.84348 7.00001 8.66667 7.00001H5.33333C5.15652 7.00001 4.98695 7.07025 4.86193 7.19527C4.7369 7.3203 4.66667 7.48987 4.66667 7.66668ZM12 4.96001C11.9931 4.89877 11.9796 4.83843 11.96 4.78001V4.72001C11.9279 4.65146 11.8852 4.58845 11.8333 4.53334V4.53334L7.83333 0.533343C7.77822 0.481488 7.71521 0.438731 7.64667 0.406677C7.62677 0.40385 7.60657 0.40385 7.58667 0.406677C7.51894 0.367838 7.44415 0.342906 7.36667 0.333344H4.66667C4.13623 0.333344 3.62753 0.544057 3.25245 0.91913C2.87738 1.2942 2.66667 1.80291 2.66667 2.33334V9.00001C2.66667 9.53044 2.87738 10.0392 3.25245 10.4142C3.62753 10.7893 4.13623 11 4.66667 11H10C10.5304 11 11.0391 10.7893 11.4142 10.4142C11.7893 10.0392 12 9.53044 12 9.00001V5.00001C12 5.00001 12 5.00001 12 4.96001ZM8 2.60668L9.72667 4.33334H8.66667C8.48986 4.33334 8.32029 4.26311 8.19526 4.13808C8.07024 4.01306 8 3.84349 8 3.66668V2.60668ZM10.6667 9.00001C10.6667 9.17682 10.5964 9.34639 10.4714 9.47141C10.3464 9.59644 10.1768 9.66668 10 9.66668H4.66667C4.48986 9.66668 4.32029 9.59644 4.19526 9.47141C4.07024 9.34639 4 9.17682 4 9.00001V2.33334C4 2.15653 4.07024 1.98696 4.19526 1.86194C4.32029 1.73691 4.48986 1.66668 4.66667 1.66668H6.66667V3.66668C6.66847 3.89411 6.70905 4.11956 6.78667 4.33334H5.33333C5.15652 4.33334 4.98695 4.40358 4.86193 4.52861C4.7369 4.65363 4.66667 4.8232 4.66667 5.00001C4.66667 5.17682 4.7369 5.34639 4.86193 5.47141C4.98695 5.59644 5.15652 5.66668 5.33333 5.66668H10.6667V9.00001Z",
                fill: "currentColor"
            }
        }
    })
};

pub static IconUpload: Component<()> = |cx| {
    cx.render(rsx! {
        span { class: "mr-1",
            svg { class: "h-3 w-3 text-indigo-300", view_box: "0 0 14 14", xmlns: "http://www.w3.org/2000/svg", fill: "none",
                path {
                    d: "M13 8.33337C12.6 8.33337 12.3333 8.60004 12.3333 9.00004V11.6667C12.3333 12.0667 12.0666 12.3334 11.6666 12.3334H2.33331C1.93331 12.3334 1.66665 12.0667 1.66665 11.6667V9.00004C1.66665 8.60004 1.39998 8.33337 0.99998 8.33337C0.59998 8.33337 0.333313 8.60004 0.333313 9.00004V11.6667C0.333313 12.8 1.19998 13.6667 2.33331 13.6667H11.6666C12.8 13.6667 13.6666 12.8 13.6666 11.6667V9.00004C13.6666 8.60004 13.4 8.33337 13 8.33337ZM4.79998 4.13337L6.33331 2.60004V9.00004C6.33331 9.40004 6.59998 9.66671 6.99998 9.66671C7.39998 9.66671 7.66665 9.40004 7.66665 9.00004V2.60004L9.19998 4.13337C9.46665 4.40004 9.86665 4.40004 10.1333 4.13337C10.4 3.86671 10.4 3.46671 10.1333 3.20004L7.46665 0.533374C7.19998 0.266707 6.79998 0.266707 6.53331 0.533374L3.86665 3.20004C3.59998 3.46671 3.59998 3.86671 3.86665 4.13337C4.13331 4.40004 4.53331 4.40004 4.79998 4.13337Z",
                    fill: "currentColor"
                }
            }
        }
    })
};

pub static IconCharts: Component<()> = |cx| {
    cx.render(rsx! {
        svg { width: "16", height: "16", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 16 16", fill: "none",
            path {
                d: "M3.99984 8.66669H1.33317C1.15636 8.66669 0.98679 8.73692 0.861766 8.86195C0.736742 8.98697 0.666504 9.15654 0.666504 9.33335V14.6667C0.666504 14.8435 0.736742 15.0131 0.861766 15.1381C0.98679 15.2631 1.15636 15.3334 1.33317 15.3334H3.99984C4.17665 15.3334 4.34622 15.2631 4.47124 15.1381C4.59627 15.0131 4.6665 14.8435 4.6665 14.6667V9.33335C4.6665 9.15654 4.59627 8.98697 4.47124 8.86195C4.34622 8.73692 4.17665 8.66669 3.99984 8.66669ZM3.33317 14H1.99984V10H3.33317V14ZM14.6665 6.00002H11.9998C11.823 6.00002 11.6535 6.07026 11.5284 6.19528C11.4034 6.32031 11.3332 6.48988 11.3332 6.66669V14.6667C11.3332 14.8435 11.4034 15.0131 11.5284 15.1381C11.6535 15.2631 11.823 15.3334 11.9998 15.3334H14.6665C14.8433 15.3334 15.0129 15.2631 15.1379 15.1381C15.2629 15.0131 15.3332 14.8435 15.3332 14.6667V6.66669C15.3332 6.48988 15.2629 6.32031 15.1379 6.19528C15.0129 6.07026 14.8433 6.00002 14.6665 6.00002ZM13.9998 14H12.6665V7.33335H13.9998V14ZM9.33317 0.666687H6.6665C6.48969 0.666687 6.32012 0.736925 6.1951 0.861949C6.07007 0.986973 5.99984 1.15654 5.99984 1.33335V14.6667C5.99984 14.8435 6.07007 15.0131 6.1951 15.1381C6.32012 15.2631 6.48969 15.3334 6.6665 15.3334H9.33317C9.50998 15.3334 9.67955 15.2631 9.80457 15.1381C9.9296 15.0131 9.99984 14.8435 9.99984 14.6667V1.33335C9.99984 1.15654 9.9296 0.986973 9.80457 0.861949C9.67955 0.736925 9.50998 0.666687 9.33317 0.666687ZM8.6665 14H7.33317V2.00002H8.6665V14Z",
                fill: "#8880EB"
            }
        }
    })
};

pub static IconTripleDots: Component<()> = |cx| {
    cx.render(rsx! {
        svg { view_box: "0 0 16 4", fill: "none", xmlns: "http://www.w3.org/2000/svg", width: "16", height: "4",
            path {
                d: "M8 0.333344C7.67037 0.333344 7.34813 0.431092 7.07405 0.614228C6.79997 0.797363 6.58635 1.05766 6.4602 1.36221C6.33406 1.66675 6.30105 2.00186 6.36536 2.32516C6.42967 2.64846 6.5884 2.94543 6.82149 3.17852C7.05458 3.41161 7.35155 3.57034 7.67485 3.63465C7.99815 3.69896 8.33326 3.66596 8.63781 3.53981C8.94235 3.41366 9.20265 3.20004 9.38578 2.92596C9.56892 2.65188 9.66667 2.32965 9.66667 2.00001C9.66667 1.55798 9.49107 1.13406 9.17851 0.821499C8.86595 0.508939 8.44203 0.333344 8 0.333344ZM2.16667 0.333344C1.83703 0.333344 1.5148 0.431092 1.24072 0.614228C0.966635 0.797363 0.753014 1.05766 0.626868 1.36221C0.500722 1.66675 0.467717 2.00186 0.532025 2.32516C0.596334 2.64846 0.755068 2.94543 0.988156 3.17852C1.22124 3.41161 1.51822 3.57034 1.84152 3.63465C2.16482 3.69896 2.49993 3.66596 2.80447 3.53981C3.10902 3.41366 3.36931 3.20004 3.55245 2.92596C3.73559 2.65188 3.83333 2.32965 3.83333 2.00001C3.83333 1.55798 3.65774 1.13406 3.34518 0.821499C3.03262 0.508939 2.6087 0.333344 2.16667 0.333344ZM13.8333 0.333344C13.5037 0.333344 13.1815 0.431092 12.9074 0.614228C12.6333 0.797363 12.4197 1.05766 12.2935 1.36221C12.1674 1.66675 12.1344 2.00186 12.1987 2.32516C12.263 2.64846 12.4217 2.94543 12.6548 3.17852C12.8879 3.41161 13.1849 3.57034 13.5082 3.63465C13.8315 3.69896 14.1666 3.66596 14.4711 3.53981C14.7757 3.41366 15.036 3.20004 15.2191 2.92596C15.4023 2.65188 15.5 2.32965 15.5 2.00001C15.5 1.55798 15.3244 1.13406 15.0118 0.821499C14.6993 0.508939 14.2754 0.333344 13.8333 0.333344Z",
                fill: "#67798E"
            }
        }
    })
};

pub static ChevronUpDown: Component<()> = |cx| {
    cx.render(rsx! {
        svg { class: "text-gray-500", xmlns: "http://www.w3.org/2000/svg", fill: "none", width: "16", height: "16", view_box: "0 0 16 16",
            path {
                d: "M10.8596 9.52667L7.99958 12.3933L5.13958 9.52667C5.01404 9.40114 4.84378 9.33061 4.66624 9.33061C4.48871 9.33061 4.31845 9.40114 4.19291 9.52667C4.06738 9.65221 3.99685 9.82247 3.99685 10C3.99685 10.1775 4.06738 10.3478 4.19291 10.4733L7.52624 13.8067C7.65115 13.9308 7.82012 14.0005 7.99624 14.0005C8.17237 14.0005 8.34134 13.9308 8.46624 13.8067L11.7996 10.4733C11.8617 10.4112 11.911 10.3374 11.9447 10.2562C11.9783 10.175 11.9956 10.0879 11.9956 10C11.9956 9.9121 11.9783 9.82505 11.9447 9.74384C11.911 9.66262 11.8617 9.58883 11.7996 9.52667C11.7374 9.46451 11.6636 9.41521 11.5824 9.38157C11.5012 9.34793 11.4142 9.33061 11.3262 9.33061C11.2383 9.33061 11.1513 9.34793 11.0701 9.38157C10.9889 9.41521 10.9151 9.46451 10.8529 9.52667H10.8596ZM5.13958 6.47334L7.99958 3.60667L10.8596 6.47334C10.9845 6.59751 11.1535 6.6672 11.3296 6.6672C11.5057 6.6672 11.6747 6.59751 11.7996 6.47334C11.9237 6.34843 11.9934 6.17946 11.9934 6.00334C11.9934 5.82722 11.9237 5.65825 11.7996 5.53334L8.46624 2.20001C8.40427 2.13752 8.33053 2.08792 8.2493 2.05408C8.16806 2.02023 8.08092 2.00281 7.99291 2.00281C7.9049 2.00281 7.81777 2.02023 7.73653 2.05408C7.65529 2.08792 7.58155 2.13752 7.51958 2.20001L4.18624 5.53334C4.06159 5.65976 3.99227 5.83052 3.99352 6.00805C3.99477 6.18559 4.06649 6.35535 4.19291 6.48001C4.31933 6.60466 4.49009 6.67398 4.66762 6.67273C4.84516 6.67148 5.01493 6.59976 5.13958 6.47334Z",
                fill: "currentColor"
            }
        }
    })
};

pub static Empty: Component<()> = |cx| {
    cx.render(rsx! {
        svg {
            class: "text-indigo-500 bg-indigo-100 block h-8 w-8 p-2 rounded",
            view_box: "0 0 20 20",
            fill: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            title { "Mobile menu" }
            path { d: "M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z" }
        }
    })
};
pub static SearchGlass: Component<()> = |cx| {
    cx.render(rsx! {
        svg { class: "h-5 w-5", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 21 21",
            path {
                d: "M20.7 19.3L17 15.6C20.1 11.7 19.5 6 15.6 2.9C11.7 -0.2 5.99999 0.5 2.89999 4.3C-0.200006 8.2 0.499995 13.9 4.29999 17C7.59999 19.6 12.3 19.6 15.6 17L19.3 20.7C19.7 21.1 20.3 21.1 20.7 20.7C21.1 20.3 21.1 19.7 20.7 19.3ZM9.99999 17C6.09999 17 2.99999 13.9 2.99999 10C2.99999 6.1 6.09999 3 9.99999 3C13.9 3 17 6.1 17 10C17 13.9 13.9 17 9.99999 17Z",
                fill: "currentColor"
            }
        }
    })
};

pub static ChatMessage: Component<()> = |cx| {
    cx.render(rsx! {
        svg { class: "h-5 w-5", view_box: "0 0 18 20", fill: "none", xmlns: "http://www.w3.org/2000/svg",
            path {
                fill: "currentColor",
                d: "M15 0H3C2.20435 0 1.44129 0.316071 0.87868 0.87868C0.316071 1.44129 0 2.20435 0 3V14C0 14.7956 0.316071 15.5587 0.87868 16.1213C1.44129 16.6839 2.20435 17 3 17H5.59L8.29 19.71C8.38344 19.8027 8.49426 19.876 8.61609 19.9258C8.73793 19.9755 8.86839 20.0008 9 20C9.23834 20 9.46886 19.9149 9.65 19.76L12.87 17H15C15.7956 17 16.5587 16.6839 17.1213 16.1213C17.6839 15.5587 18 14.7956 18 14V3C18 2.20435 17.6839 1.44129 17.1213 0.87868C16.5587 0.316071 15.7956 0 15 0ZM16 14C16 14.2652 15.8946 14.5196 15.7071 14.7071C15.5196 14.8946 15.2652 15 15 15H12.5C12.2617 15 12.0311 15.0851 11.85 15.24L9.05 17.64L6.71 15.29C6.61656 15.1973 6.50574 15.124 6.38391 15.0742C6.26207 15.0245 6.13161 14.9992 6 15H3C2.73478 15 2.48043 14.8946 2.29289 14.7071C2.10536 14.5196 2 14.2652 2 14V3C2 2.73478 2.10536 2.48043 2.29289 2.29289C2.48043 2.10536 2.73478 2 3 2H15C15.2652 2 15.5196 2.10536 15.7071 2.29289C15.8946 2.48043 16 2.73478 16 3V14Z"
            }
        }
    })
};

pub static Bell: Component<()> = |cx| {
    cx.render(rsx! {
        svg { class: "h-5 w-5", fill: "none", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 16 20",
            path {
                d: "M14 11.18V8C13.9986 6.58312 13.4958 5.21247 12.5806 4.13077C11.6655 3.04908 10.3971 2.32615 9 2.09V1C9 0.734784 8.89464 0.48043 8.70711 0.292893C8.51957 0.105357 8.26522 0 8 0C7.73478 0 7.48043 0.105357 7.29289 0.292893C7.10536 0.48043 7 0.734784 7 1V2.09C5.60294 2.32615 4.33452 3.04908 3.41939 4.13077C2.50425 5.21247 2.00144 6.58312 2 8V11.18C1.41645 11.3863 0.910998 11.7681 0.552938 12.2729C0.194879 12.7778 0.00173951 13.3811 0 14V16C0 16.2652 0.105357 16.5196 0.292893 16.7071C0.48043 16.8946 0.734784 17 1 17H4.14C4.37028 17.8474 4.873 18.5954 5.5706 19.1287C6.26819 19.6621 7.1219 19.951 8 19.951C8.8781 19.951 9.73181 19.6621 10.4294 19.1287C11.127 18.5954 11.6297 17.8474 11.86 17H15C15.2652 17 15.5196 16.8946 15.7071 16.7071C15.8946 16.5196 16 16.2652 16 16V14C15.9983 13.3811 15.8051 12.7778 15.4471 12.2729C15.089 11.7681 14.5835 11.3863 14 11.18ZM4 8C4 6.93913 4.42143 5.92172 5.17157 5.17157C5.92172 4.42143 6.93913 4 8 4C9.06087 4 10.0783 4.42143 10.8284 5.17157C11.5786 5.92172 12 6.93913 12 8V11H4V8ZM8 18C7.65097 17.9979 7.30857 17.9045 7.00683 17.7291C6.70509 17.5536 6.45451 17.3023 6.28 17H9.72C9.54549 17.3023 9.29491 17.5536 8.99317 17.7291C8.69143 17.9045 8.34903 17.9979 8 18ZM14 15H2V14C2 13.7348 2.10536 13.4804 2.29289 13.2929C2.48043 13.1054 2.73478 13 3 13H13C13.2652 13 13.5196 13.1054 13.7071 13.2929C13.8946 13.4804 14 13.7348 14 14V15Z",
                fill: "currentColor"
            }
        }
    })
};

pub static Alert: Component<()> = |cx| {
    cx.render(rsx! {
        svg { height: "20", fill: "none", width: "20", view_box: "0 0 20 20", xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M10 0C4.5 0 0 4.5 0 10C0 15.5 4.5 20 10 20C15.5 20 20 15.5 20 10C20 4.5 15.5 0 10 0ZM11 14C11 14.6 10.6 15 10 15C9.4 15 9 14.6 9 14V10C9 9.4 9.4 9 10 9C10.6 9 11 9.4 11 10V14ZM10 7C9.4 7 9 6.6 9 6C9 5.4 9.4 5 10 5C10.6 5 11 5.4 11 6C11 6.6 10.6 7 10 7Z",
                fill: "#382CDD"
            }
        }
    })
};

pub static Close: Component<()> = |cx| {
    cx.render(rsx! {
        svg { class: "text-indigo-800", fill: "none", xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 12 12", height: "12", width: "12",
            path {
                fill: "currentColor",
                d: "M6.93341 6.00008L11.1334 1.80008C11.4001 1.53341 11.4001 1.13341 11.1334 0.866748C10.8667 0.600081 10.4667 0.600081 10.2001 0.866748L6.00008 5.06675L1.80008 0.866748C1.53341 0.600081 1.13341 0.600081 0.866748 0.866748C0.600082 1.13341 0.600082 1.53341 0.866748 1.80008L5.06675 6.00008L0.866748 10.2001C0.733415 10.3334 0.666748 10.4667 0.666748 10.6667C0.666748 11.0667 0.933415 11.3334 1.33341 11.3334C1.53341 11.3334 1.66675 11.2667 1.80008 11.1334L6.00008 6.93341L10.2001 11.1334C10.3334 11.2667 10.4667 11.3334 10.6667 11.3334C10.8667 11.3334 11.0001 11.2667 11.1334 11.1334C11.4001 10.8667 11.4001 10.4667 11.1334 10.2001L6.93341 6.00008Z"
            }
        }
    })
};

// pub(super) fn IconUpload(cx: Scope) -> Element {
//     cx.render(rsx!(
//             svg { class: "h-3 w-3 text-indigo-300",
//                 fill: "none",
//                 xmlns: "http://www.w3.org/2000/svg",
//                 view_box: "0 0 14 14",
//                 path {
//                     d: "M12.9999 8.33333C12.5999 8.33333 12.3333 8.6 12.3333 9V11.6667C12.3333 12.0667 12.0666 12.3333 11.6666 12.3333H2.33325C1.93325 12.3333 1.66659 12.0667 1.66659 11.6667V9C1.66659 8.6 1.39992 8.33333 0.999919 8.33333C0.599919 8.33333 0.333252 8.6 0.333252 9V11.6667C0.333252 12.8 1.19992 13.6667 2.33325 13.6667H11.6666C12.7999 13.6667 13.6666 12.8 13.6666 11.6667V9C13.6666 8.6 13.3999 8.33333 12.9999 8.33333ZM4.79992 4.13333L6.33325 2.6V9C6.33325 9.4 6.59992 9.66667 6.99992 9.66667C7.39992 9.66667 7.66659 9.4 7.66659 9V2.6L9.19992 4.13333C9.46659 4.4 9.86659 4.4 10.1333 4.13333C10.3999 3.86667 10.3999 3.46667 10.1333 3.2L7.46659 0.533334C7.19992 0.266667 6.79992 0.266667 6.53325 0.533334L3.86659 3.2C3.59992 3.46667 3.59992 3.86667 3.86659 4.13333C4.13325 4.4 4.53325 4.4 4.79992 4.13333Z",
//                     fill: "currentColor",
//                 }
//             }
// 		))
// }

pub(super) fn IconCalendar(cx: Scope) -> Element {
    cx.render(rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                height: "14",
                view_box: "0 0 14 14",
                width: "14",
                fill: "none",
                path {
                    fill: "#E1E4E8",
                    d: "M11.6666 1.66667H10.3333V1C10.3333 0.82319 10.263 0.65362 10.138 0.528596C10.013 0.403572 9.8434 0.333334 9.66659 0.333334C9.48977 0.333334 9.32021 0.403572 9.19518 0.528596C9.07016 0.65362 8.99992 0.82319 8.99992 1V1.66667H4.99992V1C4.99992 0.82319 4.92968 0.65362 4.80466 0.528596C4.67963 0.403572 4.51006 0.333334 4.33325 0.333334C4.15644 0.333334 3.98687 0.403572 3.86185 0.528596C3.73682 0.65362 3.66659 0.82319 3.66659 1V1.66667H2.33325C1.80282 1.66667 1.29411 1.87738 0.919038 2.25245C0.543966 2.62753 0.333252 3.13623 0.333252 3.66667V11.6667C0.333252 12.1971 0.543966 12.7058 0.919038 13.0809C1.29411 13.456 1.80282 13.6667 2.33325 13.6667H11.6666C12.197 13.6667 12.7057 13.456 13.0808 13.0809C13.4559 12.7058 13.6666 12.1971 13.6666 11.6667V3.66667C13.6666 3.13623 13.4559 2.62753 13.0808 2.25245C12.7057 1.87738 12.197 1.66667 11.6666 1.66667ZM12.3333 11.6667C12.3333 11.8435 12.263 12.013 12.138 12.1381C12.013 12.2631 11.8434 12.3333 11.6666 12.3333H2.33325C2.15644 12.3333 1.98687 12.2631 1.86185 12.1381C1.73682 12.013 1.66659 11.8435 1.66659 11.6667V7H12.3333V11.6667ZM12.3333 5.66667H1.66659V3.66667C1.66659 3.48986 1.73682 3.32029 1.86185 3.19526C1.98687 3.07024 2.15644 3 2.33325 3H3.66659V3.66667C3.66659 3.84348 3.73682 4.01305 3.86185 4.13807C3.98687 4.2631 4.15644 4.33333 4.33325 4.33333C4.51006 4.33333 4.67963 4.2631 4.80466 4.13807C4.92968 4.01305 4.99992 3.84348 4.99992 3.66667V3H8.99992V3.66667C8.99992 3.84348 9.07016 4.01305 9.19518 4.13807C9.32021 4.2631 9.48977 4.33333 9.66659 4.33333C9.8434 4.33333 10.013 4.2631 10.138 4.13807C10.263 4.01305 10.3333 3.84348 10.3333 3.66667V3H11.6666C11.8434 3 12.013 3.07024 12.138 3.19526C12.263 3.32029 12.3333 3.48986 12.3333 3.66667V5.66667Z",
                }
            }
		))
}
pub(super) fn IconChevron(cx: Scope) -> Element {
    cx.render(rsx!(
            svg {
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                width: "14",
                height: "14",
                view_box: "0 0 14 14",
                path {
                    d: "M11.6666 1.66667H10.3333V1C10.3333 0.82319 10.263 0.65362 10.138 0.528596C10.013 0.403572 9.8434 0.333334 9.66659 0.333334C9.48977 0.333334 9.32021 0.403572 9.19518 0.528596C9.07016 0.65362 8.99992 0.82319 8.99992 1V1.66667H4.99992V1C4.99992 0.82319 4.92968 0.65362 4.80466 0.528596C4.67963 0.403572 4.51006 0.333334 4.33325 0.333334C4.15644 0.333334 3.98687 0.403572 3.86185 0.528596C3.73682 0.65362 3.66659 0.82319 3.66659 1V1.66667H2.33325C1.80282 1.66667 1.29411 1.87738 0.919038 2.25245C0.543966 2.62753 0.333252 3.13623 0.333252 3.66667V11.6667C0.333252 12.1971 0.543966 12.7058 0.919038 13.0809C1.29411 13.456 1.80282 13.6667 2.33325 13.6667H11.6666C12.197 13.6667 12.7057 13.456 13.0808 13.0809C13.4559 12.7058 13.6666 12.1971 13.6666 11.6667V3.66667C13.6666 3.13623 13.4559 2.62753 13.0808 2.25245C12.7057 1.87738 12.197 1.66667 11.6666 1.66667ZM12.3333 11.6667C12.3333 11.8435 12.263 12.013 12.138 12.1381C12.013 12.2631 11.8434 12.3333 11.6666 12.3333H2.33325C2.15644 12.3333 1.98687 12.2631 1.86185 12.1381C1.73682 12.013 1.66659 11.8435 1.66659 11.6667V7H12.3333V11.6667ZM12.3333 5.66667H1.66659V3.66667C1.66659 3.48986 1.73682 3.32029 1.86185 3.19526C1.98687 3.07024 2.15644 3 2.33325 3H3.66659V3.66667C3.66659 3.84348 3.73682 4.01305 3.86185 4.13807C3.98687 4.2631 4.15644 4.33333 4.33325 4.33333C4.51006 4.33333 4.67963 4.2631 4.80466 4.13807C4.92968 4.01305 4.99992 3.84348 4.99992 3.66667V3H8.99992V3.66667C8.99992 3.84348 9.07016 4.01305 9.19518 4.13807C9.32021 4.2631 9.48977 4.33333 9.66659 4.33333C9.8434 4.33333 10.013 4.2631 10.138 4.13807C10.263 4.01305 10.3333 3.84348 10.3333 3.66667V3H11.6666C11.8434 3 12.013 3.07024 12.138 3.19526C12.263 3.32029 12.3333 3.48986 12.3333 3.66667V5.66667Z",
                    fill: "#E1E4E8",
                }
            }
		))
}
pub(super) fn IconChartBars(cx: Scope) -> Element {
    cx.render(rsx!(
            svg { class: "h-4 w-4 text-indigo-500",
                view_box: "0 0 20 20",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M4.99992 10.8333H1.66659C1.44557 10.8333 1.23361 10.9211 1.07733 11.0774C0.921049 11.2337 0.833252 11.4457 0.833252 11.6667V18.3333C0.833252 18.5544 0.921049 18.7663 1.07733 18.9226C1.23361 19.0789 1.44557 19.1667 1.66659 19.1667H4.99992C5.22093 19.1667 5.43289 19.0789 5.58917 18.9226C5.74545 18.7663 5.83325 18.5544 5.83325 18.3333V11.6667C5.83325 11.4457 5.74545 11.2337 5.58917 11.0774C5.43289 10.9211 5.22093 10.8333 4.99992 10.8333ZM4.16658 17.5H2.49992V12.5H4.16658V17.5ZM18.3333 7.50001H14.9999C14.7789 7.50001 14.5669 7.5878 14.4107 7.74408C14.2544 7.90036 14.1666 8.11233 14.1666 8.33334V18.3333C14.1666 18.5544 14.2544 18.7663 14.4107 18.9226C14.5669 19.0789 14.7789 19.1667 14.9999 19.1667H18.3333C18.5543 19.1667 18.7662 19.0789 18.9225 18.9226C19.0788 18.7663 19.1666 18.5544 19.1666 18.3333V8.33334C19.1666 8.11233 19.0788 7.90036 18.9225 7.74408C18.7662 7.5878 18.5543 7.50001 18.3333 7.50001ZM17.4999 17.5H15.8333V9.16667H17.4999V17.5ZM11.6666 0.83334H8.33325C8.11224 0.83334 7.90028 0.921137 7.744 1.07742C7.58772 1.2337 7.49992 1.44566 7.49992 1.66667V18.3333C7.49992 18.5544 7.58772 18.7663 7.744 18.9226C7.90028 19.0789 8.11224 19.1667 8.33325 19.1667H11.6666C11.8876 19.1667 12.0996 19.0789 12.2558 18.9226C12.4121 18.7663 12.4999 18.5544 12.4999 18.3333V1.66667C12.4999 1.44566 12.4121 1.2337 12.2558 1.07742C12.0996 0.921137 11.8876 0.83334 11.6666 0.83334ZM10.8333 17.5H9.16658V2.50001H10.8333V17.5Z",
                    fill: "currentColor",
                }
            }
		))
}
pub(super) fn IconChevron2(cx: Scope) -> Element {
    cx.render(rsx!(
            svg { class: "h-2 w-2 text-gray-500",
                view_box: "0 0 6 10",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    fill: "currentColor",
                    d: "M1.23242 9.3689C1.06762 9.36887 0.906542 9.31997 0.769534 9.2284C0.632526 9.13683 0.525742 9.0067 0.462684 8.85445C0.399625 8.7022 0.383124 8.53467 0.415263 8.37304C0.447403 8.21141 0.526741 8.06294 0.643249 7.9464L3.58916 5L0.643224 2.05364C0.486959 1.89737 0.399171 1.68543 0.39917 1.46444C0.399169 1.24345 0.486957 1.03151 0.64322 0.875249C0.799483 0.718985 1.01142 0.631196 1.23241 0.631195C1.4534 0.631194 1.66534 0.718982 1.82161 0.875245L5.35676 4.41084C5.43416 4.48819 5.49556 4.58005 5.53745 4.68114C5.57934 4.78224 5.6009 4.8906 5.6009 5.00003C5.6009 5.10946 5.57934 5.21782 5.53745 5.31891C5.49556 5.42001 5.43416 5.51186 5.35676 5.58922L1.82161 9.12478C1.74432 9.20229 1.65249 9.26375 1.55137 9.30564C1.45026 9.34754 1.34186 9.36903 1.23242 9.3689Z",
                }
            }
		))
}
pub(super) fn IconHome(cx: Scope) -> Element {
    cx.render(rsx!(
            svg { class: "h-4 w-4 text-gray-500",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 16 18",
                path {
                    d: "M14.6666 5.66667L9.66662 1.28333C9.20827 0.873372 8.6149 0.646725 7.99996 0.646725C7.38501 0.646725 6.79164 0.873372 6.33329 1.28333L1.33329 5.66667C1.0686 5.9034 0.857374 6.1938 0.713683 6.51854C0.569993 6.84328 0.497134 7.1949 0.499957 7.55V14.8333C0.499957 15.4964 0.763349 16.1323 1.23219 16.6011C1.70103 17.0699 2.33692 17.3333 2.99996 17.3333H13C13.663 17.3333 14.2989 17.0699 14.7677 16.6011C15.2366 16.1323 15.5 15.4964 15.5 14.8333V7.54167C15.5016 7.18797 15.4282 6.83795 15.2845 6.51474C15.1409 6.19152 14.9303 5.90246 14.6666 5.66667V5.66667ZM9.66662 15.6667H6.33329V11.5C6.33329 11.279 6.42109 11.067 6.57737 10.9107C6.73365 10.7545 6.94561 10.6667 7.16662 10.6667H8.83329C9.0543 10.6667 9.26626 10.7545 9.42255 10.9107C9.57883 11.067 9.66662 11.279 9.66662 11.5V15.6667ZM13.8333 14.8333C13.8333 15.0543 13.7455 15.2663 13.5892 15.4226C13.4329 15.5789 13.221 15.6667 13 15.6667H11.3333V11.5C11.3333 10.837 11.0699 10.2011 10.6011 9.73223C10.1322 9.26339 9.49633 9 8.83329 9H7.16662C6.50358 9 5.8677 9.26339 5.39886 9.73223C4.93002 10.2011 4.66662 10.837 4.66662 11.5V15.6667H2.99996C2.77894 15.6667 2.56698 15.5789 2.4107 15.4226C2.25442 15.2663 2.16662 15.0543 2.16662 14.8333V7.54167C2.16677 7.42335 2.19212 7.30641 2.24097 7.19865C2.28982 7.09089 2.36107 6.99476 2.44996 6.91667L7.44996 2.54167C7.60203 2.40807 7.79753 2.33439 7.99996 2.33439C8.20238 2.33439 8.39788 2.40807 8.54996 2.54167L13.55 6.91667C13.6388 6.99476 13.7101 7.09089 13.7589 7.19865C13.8078 7.30641 13.8331 7.42335 13.8333 7.54167V14.8333Z",
                    fill: "currentColor",
                }
            }
		))
}