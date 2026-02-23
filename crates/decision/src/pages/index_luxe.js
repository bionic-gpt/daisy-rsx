(() => {
  const pageSpec = {
    artifacts: {
      telemetry_typewriter: {
        feeds: [
          'Reporting variance detected across operational inputs.',
          'Context convergence achieved after cross-system correlation.',
          'Decision window tightening-judgment posture required.',
        ],
      },
    },
    animations: {
      nav: {
        threshold: 0.35,
        hover_lift_px: 1,
        cta_magnetic_scale: 1.03,
      },
      hero: {
        duration_ms: 900,
        translate_y_px: 40,
      },
      diagnostic_shuffler: {
        interval_ms: 3000,
      },
      telemetry_typewriter: {
        char_interval_ms: 30,
        line_pause_ms: 900,
        cursor_blink_ms: 1000,
      },
      scheduler: {
        step_interval_ms: 2200,
        activate_delay_ms: 140,
        move_to_save_delay_ms: 500,
      },
      manifesto: {
        max_offset_px: 26,
      },
      protocol_stack: {
        dimmed_scale: 0.9,
        dimmed_blur_px: 20,
        dimmed_opacity: 0.5,
      },
    },
  };

  const validThemes = new Set([
    'midnight-luxe',
    'organic-tech',
    'brutalist-signal',
    'vapor-clinic',
  ]);
  const params = new URLSearchParams(window.location.search);
  const queryTheme = params.get('theme');
  const savedTheme = window.localStorage.getItem('decision_theme');
  const selected = validThemes.has(queryTheme || '')
    ? queryTheme
    : (validThemes.has(savedTheme || '') ? savedTheme : 'midnight-luxe');
  document.body.setAttribute('data-dl-theme', selected);
  window.localStorage.setItem('decision_theme', selected);
  document.documentElement.style.setProperty('--dl-nav-hover-lift-px', String(pageSpec.animations.nav.hover_lift_px));
  document.documentElement.style.setProperty('--dl-cta-magnetic-scale', String(pageSpec.animations.nav.cta_magnetic_scale));
  document.documentElement.style.setProperty('--dl-hero-duration-ms', String(pageSpec.animations.hero.duration_ms));
  document.documentElement.style.setProperty('--dl-hero-translate-y', `${pageSpec.animations.hero.translate_y_px}px`);
  document.documentElement.style.setProperty('--dl-cursor-blink-ms', String(pageSpec.animations.telemetry_typewriter.cursor_blink_ms));
  document.documentElement.style.setProperty('--dl-stack-dimmed-scale', String(pageSpec.animations.protocol_stack.dimmed_scale));
  document.documentElement.style.setProperty('--dl-stack-dimmed-blur-px', `${pageSpec.animations.protocol_stack.dimmed_blur_px}px`);
  document.documentElement.style.setProperty('--dl-stack-dimmed-opacity', String(pageSpec.animations.protocol_stack.dimmed_opacity));

  const hero = document.getElementById('hero');
  const shell = document.querySelector('.site-nav-shell');
  if (hero && shell) {
    const io = new IntersectionObserver((entries) => {
      const visible = entries[0]?.isIntersecting;
      document.body.classList.toggle('nav-scrolled', !visible);
    }, { threshold: pageSpec.animations.nav.threshold });
    io.observe(hero);
  }

  const stage = document.getElementById('diag-stage');
  if (stage) {
    setInterval(() => {
      const cards = Array.from(stage.querySelectorAll('.diag-item'));
      if (cards.length < 2) return;
      const last = cards[cards.length - 1];
      stage.insertBefore(last, cards[0]);
    }, pageSpec.animations.diagnostic_shuffler.interval_ms);
  }

  const feedLines = pageSpec.artifacts.telemetry_typewriter.feeds;
  const lineEl = document.getElementById('type-line');
  if (lineEl) {
    let i = 0;
    let j = 0;
    let current = feedLines[0];
    const tick = () => {
      if (j <= current.length) {
        lineEl.firstChild.textContent = current.slice(0, j);
        j += 1;
        setTimeout(tick, pageSpec.animations.telemetry_typewriter.char_interval_ms);
      } else {
        setTimeout(() => {
          i = (i + 1) % feedLines.length;
          current = feedLines[i];
          j = 0;
          tick();
        }, pageSpec.animations.telemetry_typewriter.line_pause_ms);
      }
    };
    tick();
  }

  const scheduler = document.getElementById('scheduler');
  const cursor = document.getElementById('schedule-cursor');
  if (scheduler && cursor) {
    const cells = Array.from(scheduler.querySelectorAll('.day-cell'));
    const saveBtn = document.getElementById('save-btn');
    let idx = 0;

                        const moveTo = (el) => {
                          const host = scheduler.getBoundingClientRect();
                          const box = el.getBoundingClientRect();
                          const x = box.left - host.left + box.width / 2 - 5;
                          const y = box.top - host.top + box.height / 2 - 5;
                          cursor.style.opacity = '1';
                          cursor.style.transform = `translate(${x}px, ${y}px)`;
                        };

    const step = () => {
      cells.forEach((c) => c.classList.remove('active'));
      const cell = cells[idx % cells.length];
      if (!cell) return;
      moveTo(cell);
      setTimeout(() => cell.classList.add('active'), pageSpec.animations.scheduler.activate_delay_ms);
      idx += 2;

      setTimeout(() => {
        if (saveBtn) moveTo(saveBtn);
      }, pageSpec.animations.scheduler.move_to_save_delay_ms);

      setTimeout(step, pageSpec.animations.scheduler.step_interval_ms);
    };
    step();
  }

  const parallax = document.getElementById('parallax');
  const manifesto = document.getElementById('manifesto');
  const stackCards = [
    document.getElementById('protocol-card-1'),
    document.getElementById('protocol-card-2'),
    document.getElementById('protocol-card-3'),
  ].filter(Boolean);

  const onScroll = () => {
    if (parallax && manifesto) {
      const rect = manifesto.getBoundingClientRect();
      const progress = Math.max(-0.2, Math.min(1.2, 1 - rect.top / window.innerHeight));
      const offset = progress * pageSpec.animations.manifesto.max_offset_px;
      parallax.style.transform = `translateY(${offset}px)`;
    }

    stackCards.forEach((card, i) => {
      const r = card.getBoundingClientRect();
      const threshold = 130 + i * 18;
      card.classList.toggle('dimmed', r.top < threshold);
    });
  };

  window.addEventListener('scroll', onScroll, { passive: true });
  onScroll();
})();
