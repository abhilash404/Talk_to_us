// Replace with actual user ID retrieval logic
// Section switching functionality
const sidebarLinks = document.querySelectorAll('.sidebar-link');
const sections = document.querySelectorAll('.content-section');
const pageTitle = document.getElementById('page-title');

const sectionTitles = {
    'faq': 'Commonly Asked Questions',
    'tickets': 'Your Tickets',
    'contact': 'Contact Us',
    'downloads': 'Downloads'
};
sidebarLinks.forEach(link => {
    link.addEventListener('click', function(e) {
        e.preventDefault();
        
        // Remove active class from all links
        sidebarLinks.forEach(l => l.classList.remove('active'));
        
        // Add active class to clicked link
        this.classList.add('active');
        
        // Hide all sections
        sections.forEach(section => section.classList.add('hidden'));
        
        // Show selected section
        const sectionId = this.dataset.section + '-section';
        const targetSection = document.getElementById(sectionId);
        if (targetSection) {
            targetSection.classList.remove('hidden');
        }
        
        // Update page title
        pageTitle.textContent = sectionTitles[this.dataset.section];

        ticket_render();
    });
});
// FAQ accordion functionality
const faqQuestions = document.querySelectorAll('.faq-question');
faqQuestions.forEach(question => {
    question.addEventListener('click', function() {
        const answer = this.nextElementSibling;
        const arrow = this.querySelector('span');
        
        if (answer.classList.contains('show')) {
            answer.classList.remove('show');
            arrow.textContent = '⌄';
        } else {
            answer.classList.add('show');
            arrow.textContent = '⌃';
        }
    });
});
// Ticket tabs functionality
const ticketTabs = document.querySelectorAll('#tickets-section .tab');
const ticketUnderline = document.querySelector('#tickets-section .tab-underline');

function updateTicketUnderline(activeTab) {
    const rect = activeTab.getBoundingClientRect();
    const tabsRect = activeTab.parentElement.getBoundingClientRect();
    
    ticketUnderline.style.left = (rect.left - tabsRect.left) + 'px';
    ticketUnderline.style.width = rect.width + 'px';
}
ticketTabs.forEach(tab => {
    tab.addEventListener('click', function() {
        // Remove active class from all tabs
        ticketTabs.forEach(t => t.classList.remove('active'));
        
        // Add active class to clicked tab
        this.classList.add('active');
        
        // Update underline position
        updateTicketUnderline(this);
        
        // Filter tickets
        const filter = this.dataset.filter;
        const ticketRows = document.querySelectorAll('#tickets-container .table-row');
        
        ticketRows.forEach(row => {
            const status = row.dataset.status;
            if (filter === 'all' || status === filter) {
                row.style.display = 'grid';
            } else {
                row.style.display = 'none';
            }
        });
    });
});
const searchInput = document.querySelector('.search-input');
searchInput.addEventListener('input', function() {
    const searchTerm = this.value.toLowerCase();
    const activeSection = document.querySelector('.content-section:not(.hidden)');
    
    if (activeSection.id === 'faq-section') {
        const faqItems = activeSection.querySelectorAll('.faq-item');
        faqItems.forEach(item => {
            const question = item.querySelector('.faq-question').textContent.toLowerCase();
            const answer = item.querySelector('.faq-answer').textContent.toLowerCase();
            
            if (question.includes(searchTerm) || answer.includes(searchTerm)) {
                item.style.display = 'block';
            } else {
                item.style.display = 'none';
            }
        });
    } else if (activeSection.id === 'tickets-section') {
        const ticketRows = activeSection.querySelectorAll('.table-row');
        ticketRows.forEach(row => {
            const subject = row.querySelector('.subject').textContent.toLowerCase();
            
            if (subject.includes(searchTerm)) {
                row.style.display = 'grid';
            } else {
                row.style.display = 'none';
            }
        });
    }
});
// Initialize ticket underline
setTimeout(() => {
    const activeTicketTab = document.querySelector('#tickets-section .tab.active');
    if (activeTicketTab) {
        updateTicketUnderline(activeTicketTab);
    }
}, 100);
const USER_ID = 1; // or dynamically set this

async function loadUserTickets() {
    try {
        const response = await fetch(`http://127.0.0.1:3000/tickets/${USER_ID}`);
        if (!response.ok) throw new Error("Failed to fetch tickets");
        const tickets = await response.json();

        const container = document.getElementById("tickets-container");
        container.innerHTML = "";

        if (tickets.length === 0) {
            container.innerHTML = '<div class="empty-state">No tickets found.</div>';
            return;
        }

        tickets.forEach(ticket => {
            const row = document.createElement("div");
            row.className = "table-row";
            row.dataset.status = ticket.status.toLowerCase();

            row.innerHTML = `
                <div class="subject">${ticket.title}</div>
                <div class="${statusClass(ticket.status.toLowerCase())}">${capitalize(ticket.status)}</div>
                <div class="${priorityClass(ticket.priority.toLowerCase())}">${capitalize(ticket.priority)}</div>
                <div class="created-date">${timeAgo(ticket.created_at)}</div>
            `;
            container.appendChild(row);
        });
    } catch (err) {
        document.getElementById("tickets-container").innerHTML =
            `<div class="empty-state">Error loading tickets.</div>`;
        console.error(err);
    }
}



