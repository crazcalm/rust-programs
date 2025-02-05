// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="chapter_1.html"><strong aria-hidden="true">1.</strong> Chapter 1</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.</strong> Advent of Code</div></li><li><ol class="section"><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.</strong> 2015</div></li><li><ol class="section"><li class="chapter-item expanded "><a href="advent_of_code/2015/day_1.html"><strong aria-hidden="true">2.1.1.</strong> Day 1</a></li><li class="chapter-item expanded "><a href="advent_of_code/2015/day_2.html"><strong aria-hidden="true">2.1.2.</strong> Day 2</a></li><li class="chapter-item expanded "><a href="advent_of_code/2015/day_3.html"><strong aria-hidden="true">2.1.3.</strong> Day 3</a></li><li class="chapter-item expanded "><a href="advent_of_code/2015/day_4.html"><strong aria-hidden="true">2.1.4.</strong> Day 4</a></li><li class="chapter-item expanded "><a href="advent_of_code/2015/day_5.html"><strong aria-hidden="true">2.1.5.</strong> Day 5</a></li><li class="chapter-item expanded "><a href="advent_of_code/2015/day_6.html"><strong aria-hidden="true">2.1.6.</strong> Day 6</a></li><li class="chapter-item expanded "><a href="advent_of_code/2015/day_7.html"><strong aria-hidden="true">2.1.7.</strong> Day 7</a></li><li class="chapter-item expanded "><a href="advent_of_code/2015/day_8.html"><strong aria-hidden="true">2.1.8.</strong> Day 8</a></li><li class="chapter-item expanded "><a href="advent_of_code/2015/day_9.html"><strong aria-hidden="true">2.1.9.</strong> Day 9</a></li><li class="chapter-item expanded "><a href="advent_of_code/2015/day_10.html"><strong aria-hidden="true">2.1.10.</strong> Day 10</a></li><li class="chapter-item expanded "><a href="advent_of_code/2015/day_11.html"><strong aria-hidden="true">2.1.11.</strong> Day 11</a></li><li class="chapter-item expanded "><a href="advent_of_code/2015/day_12.html"><strong aria-hidden="true">2.1.12.</strong> Day 12</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.13.</strong> Day 13</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.14.</strong> Day 14</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.15.</strong> Day 15</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.16.</strong> Day 16</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.17.</strong> Day 17</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.18.</strong> Day 18</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.19.</strong> Day 19</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.20.</strong> Day 20</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.21.</strong> Day 21</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.22.</strong> Day 22</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.23.</strong> Day 23</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.24.</strong> Day 24</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">2.1.25.</strong> Day 25</div></li></ol></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
