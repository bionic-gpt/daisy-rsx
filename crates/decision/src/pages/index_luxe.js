(() => {
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

  const hero = document.getElementById('hero');
                      const shell = document.querySelector('.site-nav-shell');
                      if (hero && shell) {
                        const io = new IntersectionObserver((entries) => {
                          const visible = entries[0]?.isIntersecting;
                          document.body.classList.toggle('nav-scrolled', !visible);
                        }, { threshold: 0.35 });
                        io.observe(hero);
                      }

                      const stage = document.getElementById('diag-stage');
                      if (stage) {
                        setInterval(() => {
                          const cards = Array.from(stage.querySelectorAll('.diag-item'));
                          if (cards.length < 2) return;
                          const last = cards[cards.length - 1];
                          stage.insertBefore(last, cards[0]);
                        }, 3000);
                      }

                      const feedLines = [
                        'Input variance detected across reporting channels.',
                        'Confidence threshold stabilized after third correlation pass.',
                        'Decision window narrowing: recommend action review.',
                      ];
                      const lineEl = document.getElementById('type-line');
                      if (lineEl) {
                        let i = 0;
                        let j = 0;
                        let current = feedLines[0];
                        const tick = () => {
                          if (j <= current.length) {
                            lineEl.firstChild.textContent = current.slice(0, j);
                            j += 1;
                            setTimeout(tick, 30);
                          } else {
                            setTimeout(() => {
                              i = (i + 1) % feedLines.length;
                              current = feedLines[i];
                              j = 0;
                              tick();
                            }, 900);
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
                          setTimeout(() => cell.classList.add('active'), 140);
                          idx += 2;

                          setTimeout(() => {
                            if (saveBtn) moveTo(saveBtn);
                          }, 500);

                          setTimeout(step, 2200);
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
                          const offset = progress * 26;
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
