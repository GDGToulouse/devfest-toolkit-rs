export const pages = [
  {
    key: "general",
    label: "General",
    menus: [
      { key: "info", label: "Information", element: "dftk-general" },
      { key: "users", label: "Users", element: "dftk-users" },
    ],
  },
  {
    key: "sessions",
    label: "Sessions",
    menus: [
      { key: "all", label: "Sessions", element: "dftk-sessions" },
      {
        key: "session-categories",
        label: "Categories",
        element: "dftk-session-categories",
      },
      {
        key: "session-formats",
        label: "Formats",
        element: "dftk-session-formats",
      },
    ],
  },
  {
    key: "speakers",
    label: "Speakers",
    menus: [{ key: "all", label: "Speakers", element: "dftk-speakers" }],
  },
  {
    key: "sponsors",
    label: "sponsors",
    menus: [
      { key: "all", label: "Sponsors", element: "dftk-sponsors" },
      {
        key: "sponsor-categories",
        label: "Categories",
        element: "dftk-sponsor-categories",
      },
    ],
  },
  {
    key: "team",
    label: "Team",
    menus: [
      { key: "all", label: "Members", element: "dftk-team" },
      { key: "member-types", label: "Types", element: "dftk-member-types" },
    ],
  },
  {
    key: "schedule",
    label: "Schedule",
    menus: [
      { key: "rooms", label: "Rooms", element: "dftk-rooms" },
      { key: "slots", label: "Slots", element: "dftk-slots" },
      { key: "schedule", label: "Schedule", element: "dftk-schedule" },
    ],
  },
];

export const defaultRoute = {
  page: pages[0],
  menu: pages[0].menus[0],
};
