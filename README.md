# Gosub GTK browser prototype

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
- [X] show/hide logging window with CTRL-L

TODO Gosub engine implementation (assumes async eventloop is implemented in gosub engine):
- [ ] Make sure we can use vello in a window for drawing
- [ ] enable engine to parse html
- [ ] make sure render backend can render html
- [ ] scrollable html 
- [ ] html per tab
- [ ] make use of the gosub eventloop for tasks