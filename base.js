document.addEventListener('DOMContentLoaded', () => {
    const tabs = document.querySelectorAll('.tab');
    const sections = {
        'Config': document.getElementById('Config'),
        'Presentation': document.getElementById('Prez'),
        'Stylesheet': document.getElementById('Stylesheet'),
        'Assets': document.getElementById('Assets'),
        'Preview': document.getElementById('Preview')
    };

    // Masquer toutes les sections au démarrage
    Object.values(sections).forEach(section => {
        if (section) {
            section.style.display = 'none';
        }
    });

    // Afficher la section Config par défaut
    if (sections.Config) sections.Config.style.display = 'block';

    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            // 1. Changer onglet actif
            tabs.forEach(t => t.classList.remove('active'));
            tab.classList.add('active');

            // 2. Masquer toutes les sections
            Object.values(sections).forEach(section => {
                if (section) {
                    section.style.display = 'none';
                }
            });

            // 3. Afficher la section correspondante
            const targetSection = sections[tab.textContent];
            if (targetSection) {
                targetSection.style.display = 'block';
            }

            console.log(`Onglet: ${tab.textContent}`);
        });
    });
});