tab todo:


Have the following API:

- Create Gosub Tab. This defines our tab (pinned, title, favicon etc).
- Place the gosub tab into the tab manager. This sets the tab as dirty.
- Refresh the tab manager. This will update the gtknotebook with the tabs.
- As soon as a tab is changed (moved around, pinned, etc), the tab is marked dirty and it will be updated.

Basically: the gtknotebook is the view, the tab manager is the model. The tab manager is responsible for updating the view.
Any signals from the gtknotebook tabs should be send to the tab manager so it can update stuff.



TODO Gui specific stuff:
- [ ] Implement the tab manager
- [ ] Make async work, so we can init a tab with a spinner, async load favicon, and display favicon once loaded
- [ ] Implement url loader in searchbar and CTRL-T for new tab
- [ ] implement closing tab
- [ ] implement pinned tabs (should be working but no way to pin them yet)
- [ ] implement tab dragging and sorting
- [ ] change title of window based on the title of the tab
- [ ] implement url history (per tab)
- [ ] implement scroller for logging window
- [ ] show/hide logging window with F1 (or some key)

TODO Gosub engine implementation (assumes async eventloop is implemented in gosub engine):
- [ ] Make sure we can use vello in a window for drawing
- [ ] enable engine to parse html
- [ ] make sure render backend can render html
- [ ] scrollable html 
- [ ] html per tab
- [ ] make use of the gosub eventloop for tasks