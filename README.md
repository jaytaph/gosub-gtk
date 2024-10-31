tab todo:


Have the following API:

- Create Gosub Tab. This defines our tab (pinned, title, favicon etc).
- Place the gosub tab into the tab manager. This sets the tab as dirty.
- Refresh the tab manager. This will update the gtknotebook with the tabs.
- As soon as a tab is changed (moved around, pinned, etc), the tab is marked dirty and it will be updated.

Basically: the gtknotebook is the view, the tab manager is the model. The tab manager is responsible for updating the view.
Any signals from the gtknotebook tabs should be send to the tab manager so it can update stuff.