document.getElementById('ticketForm').addEventListener('submit', async function(e) {
    e.preventDefault();
    const title = document.getElementById('title').value.trim();
    const description = document.getElementById('description').value.trim
    const token = window.globalToken || localStorage.getItem("token");
    const successMessage = document.getElementById('successMessage');
    if (!title || !description) {
        successMessage.textContent = "Please fill out all required fields.";
        successMessage.style.background = "#f8d7da";
        successMessage.style.color = "#721c24";
        successMessage.style.borderColor = "#f5c6cb";
        successMessage.classList.add('show');
        setTimeout(() => { successMessage.classList.remove('show'); }, 3000);
        return;
    }

    if (!token) {
        successMessage.textContent = "Not logged in! Please sign in first.";
        successMessage.style.background = "#f8d7da";
        successMessage.style.color = "#721c24";
        successMessage.style.borderColor = "#f5c6cb";
        successMessage.classList.add('show');
        setTimeout(() => { successMessage.classList.remove('show'); }, 3000);
        return;
    }

    // Compose data for backend
    const body = {
        title: title,
        description: description,
        priority: "low"
        // add priority/default here if you have it
    }
  
    try {
        const response = await fetch("http://127.0.0.1:3000/create", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Authorization": "Bearer " + token
            },
            body: JSON.stringify(body)
        });
        let data = '';
        try { data = await response.json(); } catch { data = await response.text(); }
        if (response.ok) {
            successMessage.textContent = "✅ Ticket created successfully! You will receive a confirmation email shortly.";
            successMessage.style.background = "#d4edda";
            successMessage.style.color = "#155724";
            successMessage.style.borderColor = "#c3e6cb";
            successMessage.classList.add('show');
            document.getElementById('ticketForm').reset();
            setTimeout(() => {
                window.location.href = "/frontend/user_dashboard.html";
            }, 2000);
        } else {
            successMessage.textContent = data && data.message ? data.message : (typeof data === "string" ? data : "Failed to create ticket.");
            successMessage.style.background = "#f8d7da";
            successMessage.style.color = "#721c24";
            successMessage.style.borderColor = "#f5c6cb";
            successMessage.classList.add('show');
            setTimeout(() => { successMessage.classList.remove('show'); }, 4000);
        }
    } catch (err) {
        successMessage.textContent = "Network or server error.";
        successMessage.style.background = "#f8d7da";
        successMessage.style.color = "#721c24";
        successMessage.style.borderColor = "#f5c6cb";
        successMessage.classList.add('show');
        setTimeout(() => { successMessage.classList.remove('show'); }, 4000);
    }

function resetForm() {
    document.getElementById('ticketForm').reset();
}
});