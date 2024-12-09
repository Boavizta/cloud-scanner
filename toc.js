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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="intro.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Tutorials 🚀</li><li class="chapter-item expanded "><a href="tutorials/quickstart-docker.html"><strong aria-hidden="true">1.</strong> Quickstart - CLI (docker 🐳)</a></li><li class="chapter-item expanded "><a href="tutorials/quickstart-dashboard-docker.html"><strong aria-hidden="true">2.</strong> Quickstart - Dashboard (docker 🐳)</a></li><li class="chapter-item expanded "><a href="tutorials/quickstart-rust-cli.html"><strong aria-hidden="true">3.</strong> Quickstart - CLI (cargo 🦀)</a></li><li class="chapter-item expanded "><a href="tutorials/quickstart-serverless.html"><strong aria-hidden="true">4.</strong> Quickstart as serverless ⚡</a></li><li class="chapter-item expanded affix "><li class="part-title">Explanations</li><li class="chapter-item expanded "><a href="explanations/methodology.html"><strong aria-hidden="true">5.</strong> Methodology</a></li><li class="chapter-item expanded "><a href="explanations/processing-workload.html"><strong aria-hidden="true">6.</strong> How we process workload</a></li><li class="chapter-item expanded "><a href="explanations/block-storage.html"><strong aria-hidden="true">7.</strong> Estimating Block Storage </a></li><li class="chapter-item expanded affix "><li class="part-title">How-to guides</li><li class="chapter-item expanded "><a href="how-to/estimate-from-existing-inventory-file.html"><strong aria-hidden="true">8.</strong> Estimate the impacts of an existing inventory</a></li><li class="chapter-item expanded "><a href="how-to/simulate-impacts-of-an-inventory.html"><strong aria-hidden="true">9.</strong> Simulate impacts of an inventory</a></li><li class="chapter-item expanded "><a href="how-to/building-cli.html"><strong aria-hidden="true">10.</strong> Building CLI</a></li><li class="chapter-item expanded "><a href="how-to/docker-guide.html"><strong aria-hidden="true">11.</strong> Run as docker</a></li><li class="chapter-item expanded "><a href="how-to/deploy-sls.html"><strong aria-hidden="true">12.</strong> Deploy serverless</a></li><li class="chapter-item expanded "><a href="how-to/passing-aws-credentials.html"><strong aria-hidden="true">13.</strong> AWS authentication</a></li><li class="chapter-item expanded "><a href="how-to/set-up-dashboard.html"><strong aria-hidden="true">14.</strong> Setup monitoring dashboard</a></li><li class="chapter-item expanded "><a href="how-to/filter-by-tags.html"><strong aria-hidden="true">15.</strong> Filtering by tags</a></li><li class="chapter-item expanded "><a href="how-to/using-private-boaviztapi.html"><strong aria-hidden="true">16.</strong> Using a private instance of Boavizta API</a></li><li class="chapter-item expanded affix "><li class="part-title">Reference</li><li class="chapter-item expanded "><a href="reference/FAQ.html"><strong aria-hidden="true">17.</strong> FAQ 💡</a></li><li class="chapter-item expanded "><a href="reference/common-issues.html"><strong aria-hidden="true">18.</strong> Common issues 🧰</a></li><li class="chapter-item expanded "><a href="reference/cli-options.html"><strong aria-hidden="true">19.</strong> CLI options</a></li><li class="chapter-item expanded "><a href="reference/env-vars.html"><strong aria-hidden="true">20.</strong> Environment variables</a></li><li class="chapter-item expanded "><a href="reference/openapi-server-mode.html"><strong aria-hidden="true">21.</strong> OpenAPI specification in server mode</a></li><li class="chapter-item expanded "><a href="reference/output-data.html"><strong aria-hidden="true">22.</strong> Output data</a></li><li class="chapter-item expanded "><a href="reference/serverless-design.html"><strong aria-hidden="true">23.</strong> Serverless design</a></li><li class="chapter-item expanded "><a href="reference/limits.html"><strong aria-hidden="true">24.</strong> Limitations</a></li><li class="chapter-item expanded "><a href="reference/testing.html"><strong aria-hidden="true">25.</strong> Unit tests</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
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
