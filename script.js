document.addEventListener('DOMContentLoaded', function() {
    const banner = document.querySelector('.banner');
    const heroActions = document.querySelector('.hero-actions');
    const dashboardTabs = document.querySelectorAll('.tab');
    
    if (banner) {
        banner.addEventListener('click', function() {
            console.log('Banner clicked - redirect to announcement');
        });
        
        banner.style.cursor = 'pointer';
    }
    
    if (heroActions) {
        const primaryBtn = heroActions.querySelector('.btn-primary');
        const secondaryBtn = heroActions.querySelector('.btn-secondary');
        
        if (primaryBtn) {
            primaryBtn.addEventListener('click', function(e) {
                e.preventDefault();
                console.log('Merge my first PR clicked');
                alert('This would start the PR merge process!');
            });
        }
        
        if (secondaryBtn) {
            secondaryBtn.addEventListener('click', function(e) {
                e.preventDefault();
                console.log('Book a demo clicked');
                alert('This would open the demo booking form!');
            });
        }
    }
    
    dashboardTabs.forEach(tab => {
        tab.addEventListener('click', function() {
            dashboardTabs.forEach(t => t.classList.remove('active'));
            this.classList.add('active');
            console.log(`Switched to ${this.textContent} tab`);
        });
    });
    
    const addIntegrationBtn = document.querySelector('.add-integration-btn');
    if (addIntegrationBtn) {
        addIntegrationBtn.addEventListener('click', function() {
            console.log('Add integration clicked');
            alert('This would open the integration setup!');
        });
    }
    
    window.addEventListener('scroll', function() {
        const header = document.querySelector('.header');
        if (window.scrollY > 100) {
            header.style.background = 'rgba(255, 255, 255, 0.98)';
        } else {
            header.style.background = 'rgba(255, 255, 255, 0.95)';
        }
    });
});