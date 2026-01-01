import { createRouter, createWebHashHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";
import SelectDatabase from "@/views/SelectDatabase.vue";
import ConnectionDetails from "@/views/ConnectionDetails.vue";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      name: "Home",
      component: HomeView,
    },
    {
      path: "/connect",
      name: "Hello",
      component: SelectDatabase,
    },
    {
      path: "/connect/:database",
      name: "ConnectionDetails",
      component: ConnectionDetails,
      props: true,
    },
  ],
});

export default router;
